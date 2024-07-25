#[cfg(test)]
mod tests {
    use gstd::prelude::*;
    use pebbles_game_io::*;


    fn create_system_and_user() -> (System, u64) {
        let sys = System::new();
        sys.init_logger();
        let user_id = 1;
        sys.mint_to(user_id, 10000000);
        (sys, user_id)
    }
}


