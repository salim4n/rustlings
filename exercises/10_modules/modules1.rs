// TODO: Fix the compiler error about calling a private function.
mod sausage_factory {
    // Ne laisse personne en dehors de ce module voir cela !
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
