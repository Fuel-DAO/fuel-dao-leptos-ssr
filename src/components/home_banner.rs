use chrono::{DateTime, Local};
use leptos::*;

use crate::state::checkout_state::CheckoutState;


#[component]
pub fn HomeBanner(is_root: bool) -> impl IntoView {
    let image_url = "/public/img/fuel-home.jpeg";
    let root_class = if is_root {"w-screen h-screen bg-cover bg-center"} else {"w-screen h-1/2 bg-cover bg-center"};
    view! {
        <div
            class=move||root_class
            style=format!("background-image: url('{}');", image_url)
        >
        <div class="absolute inset-0 bg-black opacity-50"></div>


        <div class="absolute flex flex-col w-screen">
        
        <div class="flex w-full h-16 items-center" style="margin-left:10px; margin-right:10px">
        // Logo image (1/4 width)
        <div class="w-1/4 flex justify-center ">
        <a href="/"><img src="/public/img/fueldao.svg" alt="App Logo" class="h-12" /></a>
        </div>
        
        // Middle image (1/2 width)
        <div class="w-1/2 h-16 flex justify-center bg-cover bg-center hidden  md:flex"
        style=format!("background-image: url('{}');", "/public/img/header.svg")
        >
        </div>
        
        // Empty component (1/4 width)
        <div class="w-1/4">
        </div>
        </div>  
        <div class="text-white flex flex-col p-4 flex-grow" style="margin-left:35px; margin-bottom:10px">
        <Show when=move||is_root>
            <div class="text-left">
            <p class="text-3xl md:text-5xl lg:text-7xl font-bold">Explore your place</p>
            <p class="text-3xl md:text-5xl lg:text-7xl font-bold">to stay</p>
            </div>
        </Show>
        </div>  
        // <SearchBar is_root/>
        // <div class="flex w-full">
        //             <div class="w-4/5">
        //                 <SearchBar />
        //             </div>
        //             <div class="w-1/5"></div> 
        // </div>
        <div class="flex w-full">
                    <div class="w-4/5 hidden md:block" style="margin-left: 35px;">
                        <SearchBar is_root/>
                    </div>
                    <div class="w-full md:w-1/5">
                        <div class="flex justify-center md:justify-end md:hidden">
                            <SearchBar is_root/>
                        </div>
                    </div> 
                </div>
        <Show when=move||is_root>
            <QualityService />
        </Show>
        </div>
        </div>
        
    }
    
}


#[component]
fn QualityService() -> impl IntoView {
    view! {
        <div class="flex w-full justify-center lg:justify-end mt-8 lg:mt-16">
            // Spacer for mobile screens
            <div class="hidden lg:w-1/3 lg:block"></div> 

            <div class="text-white p-2 transform rotate-0 lg:p-4 lg:mr-10 lg:w-[431.3px] lg:h-[185px]">
                <p class="pl-4 font-bold text-left font-poppins tracking-[0.075px] before:content-[''] before:absolute before:top-0 before:left-0 before:w-[4px] before:h-full before:bg-white lg:text-[27px] lg:leading-[35.1px]">
                    "Premium Electric SUV voted ‘Best Family Electric Car’, at your fingertips."
                </p>
                
                <p class="p-4">Quality service with FuelDAO, guaranteed</p>
            </div>
        </div>
    }
}


#[component]
pub fn SearchBar(is_root: bool) -> impl IntoView {
    let checkout_state = CheckoutState::get();

    let start_time = move ||  checkout_state.pickup_date_formatted.get();
    let end_time = move || checkout_state.return_date_formatted.get();

    let min_start_date = create_rw_signal( DateTime::parse_from_str(&Local::now().to_string(), "%Y-%m-%d %H:%M:%S%.f %:z").map_or("".to_string(), |f| f.format("%Y-%m-%dT%H:%M").to_string()) );


    
    let (get_pickup_time_value, set_pickup_time_value) = create_signal(String::new());
    let (_, set_return_time_value) = create_signal(String::new());
    let class = if is_root {
        "bg-black bg-opacity-75 rounded-lg p-4 flex flex-col md:flex-row items-center justify-around w-4/5 mx-auto md:mx-8 space-y-4 md:space-y-0 md:space-x-4"
    } else {
        "bg-black bg-opacity-75 rounded-lg p-4 flex flex-col md:flex-row items-center justify-around w-4/5 mb-8 mx-auto space-y-4 md:space-y-0 md:space-x-4" 
    };
    view! { 
        <div class=class>
            // Location input
            <div class="flex items-center bg-gray-800 rounded-lg px-4 py-2 w-full md:w-auto">
                <i class="fas fa-search text-white mr-2"></i>
                <input
                    type="text"
                    value= "Bengaluru"
                    placeholder="Add your location"
                    class="bg-transparent text-white focus:outline-none w-full" list="cities"
                />
                    <datalist id="cities">
                        <option value="Bengaluru"></option>
                    </datalist>
            </div>

            // Pickup Date and Time
            <div class="flex items-center bg-gray-800 rounded-lg px-4 py-2 w-full md:w-auto">
                <i class="fas fa-calendar-alt text-white mr-2"></i>
                <input
                    type="datetime-local"
                    placeholder="Pickup Date and Time"
                    class="bg-transparent text-white focus:outline-none w-full"
                    on:input=move |ev| {
                        ev.prevent_default();
                        let value = event_target_value(&ev);
                        set_pickup_time_value.set(value.clone());
                        CheckoutState::set_pickup_date_value_formatted(value);
                    }
                    value=move || start_time.clone()
                    min=move|| min_start_date.get().to_string()
                    max=end_time.clone()
                />
            </div>

            // Return Date and Time
            <div class="flex items-center bg-gray-800 rounded-lg px-4 py-2 w-full md:w-auto">
                <i class="fas fa-calendar-alt text-white mr-2"></i>
                <input
                    type="datetime-local"
                    placeholder="Return Date and Time"
                    class="bg-transparent text-white focus:outline-none w-full"
                    min= move||get_pickup_time_value.get().to_string()
                    on:input=move |ev| {
                        ev.prevent_default();
                        let value = event_target_value(&ev);
                        set_return_time_value.set(value.clone());
                        CheckoutState::set_return_date_value_formatted(value);
                    }
                    value=end_time.clone()
                />
            </div>

            // Search Button
            <a  href="/search" class="bg-green-500 text-center text-white rounded-lg px-8 py-2 w-full md:w-auto hover:bg-green-600">
                Search
            </a>
        </div>
    }
}

