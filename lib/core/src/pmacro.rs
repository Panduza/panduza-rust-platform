#[macro_export]
macro_rules! spawn_loop {
    ($device:ident, $body:expr) => {
        $device
            .spawn(async move {
                loop {
                    $body
                }
            })
            .await
    };
}

#[macro_export]
macro_rules! on_command {
    ($attribute:ident, $body:expr) => {
        $attribute.wait_one_command_then($body).await?
    };
}

#[macro_export]
macro_rules! spawn_on_command {
    ($device:ident, $attribute:ident, $callback:expr) => {
        $device
            .spawn(async move {
                loop {
                    $attribute.wait_one_command_then($callback).await?
                }
            })
            .await
    };
}
