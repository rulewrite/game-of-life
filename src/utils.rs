pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // `console_error_panic_hook` 피쳐를 활성화하면 초기화 중에 `set_panic_hook` 함수를 한번 이상 호출 할 수 있다.
    // 그러면 코드가 패닉이 됐을 때 더 나은 에러 메시지를 받을 수 있다.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
