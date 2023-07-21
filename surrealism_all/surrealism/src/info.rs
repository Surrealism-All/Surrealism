macro_rules! success_msg {
    ($Name:ident,$V:expr) => {
        pub const $Name:&str = $V;
    };

}

success_msg!(INIT_LOGGER,"Init Service : `Log Service` Successfully!");
success_msg!(INIT_CONFIG,"Init Service : `Config Service` Successfully!");