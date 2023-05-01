mod pizza_order{
    pub struct Pizza{
        pub dough: String,
        pub cheese: String, 
        pub topping: String,
    }

    impl Pizza{
        pub fn lunch(topping: &str) -> Pizza{
            Pizza{
                dough: String::from("Regular Dough"),
                cheese: String::from("Mozzarella"),
                topping: String::from(topping),
            }
        }
    }

    pub mod help_customer{
        fn seat_at_table(){
            println!("Customer seated at the table");
        }

        pub fn take_order(){
            seat_at_table();
            // Super is used to call function in above module
            let cust_pizza: super::Pizza =  
               super::Pizza::lunch("Veggies");
            serve_customer(cust_pizza);
        }

        fn serve_customer(cust_pizza: super::Pizza){
            println!("The customer is served pizza with {}", cust_pizza.topping);
        }
    }
}


// Calling a function from module
pub fn order_food(){
    crate::restaurant::pizza_order::help_customer::take_order();
}