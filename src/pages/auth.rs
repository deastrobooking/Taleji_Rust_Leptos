use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use crate::models::{LoginInput, RegisterInput, AuthResponse};

#[cfg(feature = "ssr")]
use crate::{
    db::Db,
    auth::AuthService,
    error::{AppError, log_error, validation::validate_input},
};

#[server(LoginUser, "/api")]
pub async fn login_user(input: LoginInput) -> Result<AuthResponse, ServerFnError> {
    let db = expect_context::<Db>();
    let auth_service = AuthService::new();

    // Validate input
    validate_input(&input).map_err(ServerFnError::from)?;

    let response = auth_service.login_user(&db, input).await.map_err(|e| {
        log_error(&e, "User login failed");
        ServerFnError::from(e)
    })?;

    tracing::info!("User authenticated: {}", response.user.username);
    Ok(response)
}

#[server(RegisterUser, "/api")]
pub async fn register_user(input: RegisterInput) -> Result<AuthResponse, ServerFnError> {
    let db = expect_context::<Db>();
    let auth_service = AuthService::new();

    // Validate input
    validate_input(&input).map_err(ServerFnError::from)?;

    let response = auth_service.register_user(&db, input).await.map_err(|e| {
        log_error(&e, "User registration failed");
        ServerFnError::from(e)
    })?;

    tracing::info!("User registered: {}", response.user.username);
    Ok(response)
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let (login_input, set_login_input) = create_signal(LoginInput {
        email_or_username: String::new(),
        password: String::new(),
        remember_me: Some(false),
    });
    
    let (error_message, set_error_message) = create_signal(None::<String>);
    let (loading, set_loading) = create_signal(false);

    let login_action = create_action(move |input: &LoginInput| {
        let input = input.clone();
        async move {
            set_loading.set(true);
            set_error_message.set(None);
            
            match login_user(input).await {
                Ok(response) => {
                    // Store token in localStorage and redirect
                    web_sys::window()
                        .and_then(|w| w.local_storage().ok().flatten())
                        .map(|storage| {
                            storage.set_item("auth_token", &response.token).ok();
                            storage.set_item("user_data", &serde_json::to_string(&response.user).unwrap_or_default()).ok();
                        });
                    
                    // Redirect to home page
                    let navigate = use_navigate();
                    navigate("/", Default::default());
                },
                Err(e) => {
                    set_error_message.set(Some(e.to_string()));
                }
            }
            set_loading.set(false);
        }
    });

    view! {
        <Title text="Login - Taleji" />
        <Meta name="description" content="Login to your Taleji account" />
        
        <div class="auth-container">
            <div class="auth-card">
                <h1>"Login to Taleji"</h1>
                
                <Show when=move || error_message.get().is_some()>
                    <div class="error-message">
                        {move || error_message.get().unwrap_or_default()}
                    </div>
                </Show>

                <form on:submit=move |ev| {
                    ev.prevent_default();
                    login_action.dispatch(login_input.get());
                }>
                    <div class="form-group">
                        <label for="email_or_username">"Email or Username"</label>
                        <input
                            type="text"
                            id="email_or_username"
                            required
                            prop:value=move || login_input.get().email_or_username
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_login_input.update(|input| input.email_or_username = value);
                            }
                        />
                    </div>

                    <div class="form-group">
                        <label for="password">"Password"</label>
                        <input
                            type="password"
                            id="password"
                            required
                            prop:value=move || login_input.get().password
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_login_input.update(|input| input.password = value);
                            }
                        />
                    </div>

                    <div class="form-group checkbox">
                        <input
                            type="checkbox"
                            id="remember_me"
                            prop:checked=move || login_input.get().remember_me.unwrap_or(false)
                            on:change=move |ev| {
                                let checked = event_target_checked(&ev);
                                set_login_input.update(|input| input.remember_me = Some(checked));
                            }
                        />
                        <label for="remember_me">"Remember me"</label>
                    </div>

                    <button 
                        type="submit" 
                        class="btn btn-primary"
                        disabled=move || loading.get()
                    >
                        {move || if loading.get() { "Logging in..." } else { "Login" }}
                    </button>
                </form>

                <div class="auth-links">
                    <p>
                        "Don't have an account? "
                        <a href="/register">"Sign up"</a>
                    </p>
                    <p>
                        <a href="/forgot-password">"Forgot your password?"</a>
                    </p>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    let (register_input, set_register_input) = create_signal(RegisterInput {
        username: String::new(),
        email: String::new(),
        password: String::new(),
        confirm_password: String::new(),
        display_name: String::new(),
    });
    
    let (error_message, set_error_message) = create_signal(None::<String>);
    let (loading, set_loading) = create_signal(false);

    let register_action = create_action(move |input: &RegisterInput| {
        let input = input.clone();
        async move {
            set_loading.set(true);
            set_error_message.set(None);
            
            match register_user(input).await {
                Ok(response) => {
                    // Store token in localStorage and redirect
                    web_sys::window()
                        .and_then(|w| w.local_storage().ok().flatten())
                        .map(|storage| {
                            storage.set_item("auth_token", &response.token).ok();
                            storage.set_item("user_data", &serde_json::to_string(&response.user).unwrap_or_default()).ok();
                        });
                    
                    // Redirect to home page
                    let navigate = use_navigate();
                    navigate("/", Default::default());
                },
                Err(e) => {
                    set_error_message.set(Some(e.to_string()));
                }
            }
            set_loading.set(false);
        }
    });

    view! {
        <Title text="Register - Taleji" />
        <Meta name="description" content="Create your Taleji account" />
        
        <div class="auth-container">
            <div class="auth-card">
                <h1>"Join Taleji"</h1>
                
                <Show when=move || error_message.get().is_some()>
                    <div class="error-message">
                        {move || error_message.get().unwrap_or_default()}
                    </div>
                </Show>

                <form on:submit=move |ev| {
                    ev.prevent_default();
                    register_action.dispatch(register_input.get());
                }>
                    <div class="form-group">
                        <label for="username">"Username"</label>
                        <input
                            type="text"
                            id="username"
                            required
                            prop:value=move || register_input.get().username
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_register_input.update(|input| input.username = value);
                            }
                        />
                    </div>

                    <div class="form-group">
                        <label for="email">"Email"</label>
                        <input
                            type="email"
                            id="email"
                            required
                            prop:value=move || register_input.get().email
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_register_input.update(|input| input.email = value);
                            }
                        />
                    </div>

                    <div class="form-group">
                        <label for="display_name">"Display Name"</label>
                        <input
                            type="text"
                            id="display_name"
                            required
                            prop:value=move || register_input.get().display_name
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_register_input.update(|input| input.display_name = value);
                            }
                        />
                    </div>

                    <div class="form-group">
                        <label for="password">"Password"</label>
                        <input
                            type="password"
                            id="password"
                            required
                            prop:value=move || register_input.get().password
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_register_input.update(|input| input.password = value);
                            }
                        />
                    </div>

                    <div class="form-group">
                        <label for="confirm_password">"Confirm Password"</label>
                        <input
                            type="password"
                            id="confirm_password"
                            required
                            prop:value=move || register_input.get().confirm_password
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                set_register_input.update(|input| input.confirm_password = value);
                            }
                        />
                    </div>

                    <button 
                        type="submit" 
                        class="btn btn-primary"
                        disabled=move || loading.get()
                    >
                        {move || if loading.get() { "Creating account..." } else { "Create Account" }}
                    </button>
                </form>

                <div class="auth-links">
                    <p>
                        "Already have an account? "
                        <a href="/login">"Sign in"</a>
                    </p>
                </div>
            </div>
        </div>
    }
}