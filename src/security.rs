#[cfg(feature = "ssr")]
use axum::{
    http::{header, HeaderValue, Request, StatusCode},
    middleware::Next,
    response::Response,
};
#[cfg(feature = "ssr")]
use headers::{HeaderMapExt, Origin, Referer};
#[cfg(feature = "ssr")]
use std::collections::HashMap;
#[cfg(feature = "ssr")]
use tokio::sync::RwLock;
#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
use std::time::{Duration, Instant};

/// Security headers middleware
#[cfg(feature = "ssr")]
pub async fn security_headers<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let mut response = next.run(request).await;
    
    let headers = response.headers_mut();
    
    // Security headers
    headers.insert(
        header::X_FRAME_OPTIONS,
        HeaderValue::from_static("DENY"),
    );
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        "X-XSS-Protection",
        HeaderValue::from_static("1; mode=block"),
    );
    headers.insert(
        header::STRICT_TRANSPORT_SECURITY,
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    );
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(
            "default-src 'self'; \
            script-src 'self' 'unsafe-inline' 'unsafe-eval'; \
            style-src 'self' 'unsafe-inline'; \
            img-src 'self' data: https:; \
            font-src 'self'; \
            connect-src 'self'; \
            frame-ancestors 'none';"
        ),
    );
    headers.insert(
        "Referrer-Policy",
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(
        "Permissions-Policy",
        HeaderValue::from_static(
            "camera=(), microphone=(), geolocation=(), \
            payment=(), usb=(), magnetometer=(), gyroscope=()"
        ),
    );

    Ok(response)
}

/// Simple CSRF protection middleware
#[cfg(feature = "ssr")]
pub async fn csrf_protection<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // Skip CSRF for GET, HEAD, OPTIONS requests
    if matches!(
        request.method().as_str(),
        "GET" | "HEAD" | "OPTIONS"
    ) {
        return Ok(next.run(request).await);
    }

    let headers = request.headers();
    
    // Check for proper Origin or Referer headers
    let origin = headers.typed_get::<Origin>();
    let referer = headers.typed_get::<Referer>();
    
    let valid_origin = origin
        .map(|o| o.hostname() == "localhost" || o.hostname().ends_with(".your-domain.com"))
        .unwrap_or(false);
        
    let valid_referer = referer
        .map(|r| r.hostname() == "localhost" || r.hostname().ends_with(".your-domain.com"))
        .unwrap_or(false);

    if !valid_origin && !valid_referer {
        tracing::warn!("CSRF protection triggered: invalid origin/referer");
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}

/// Rate limiting middleware
#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<RwLock<HashMap<String, (u32, Instant)>>>,
    max_requests: u32,
    window: Duration,
}

#[cfg(feature = "ssr")]
impl RateLimiter {
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            requests: Arc::new(RwLock::new(HashMap::new())),
            max_requests,
            window: Duration::from_secs(window_seconds),
        }
    }

    pub async fn check_rate_limit<B>(
        &self,
        request: Request<B>,
        next: Next<B>,
    ) -> Result<Response, StatusCode> {
        let client_ip = request
            .headers()
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.split(',').next().unwrap_or("").trim())
            .unwrap_or("unknown")
            .to_string();

        let now = Instant::now();
        let mut requests = self.requests.write().await;

        // Clean up old entries
        requests.retain(|_, (_, timestamp)| now.duration_since(*timestamp) < self.window);

        // Check current client
        let (count, first_request) = requests.entry(client_ip.clone()).or_insert((0, now));

        if now.duration_since(*first_request) >= self.window {
            // Reset window
            *count = 1;
            *first_request = now;
        } else {
            *count += 1;
        }

        if *count > self.max_requests {
            tracing::warn!(
                client_ip = %client_ip,
                count = *count,
                "Rate limit exceeded"
            );
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }

        Ok(next.run(request).await)
    }
}

/// Request ID middleware for tracing
#[cfg(feature = "ssr")]
pub async fn request_id<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let request_id = uuid::Uuid::new_v4().to_string();
    
    // Add request ID to headers for downstream processing
    request.headers_mut().insert(
        "x-request-id",
        HeaderValue::from_str(&request_id).unwrap(),
    );

    let mut response = next.run(request).await;
    
    // Add request ID to response headers
    response.headers_mut().insert(
        "x-request-id",
        HeaderValue::from_str(&request_id).unwrap(),
    );

    Ok(response)
}