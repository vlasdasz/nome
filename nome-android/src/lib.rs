#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: nome::AndroidApp) {
    nome::start_nome(app);
}
