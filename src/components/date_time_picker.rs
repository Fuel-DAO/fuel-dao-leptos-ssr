use chrono::{DateTime, Local};
use leptos::*;
use leptos_icons::Icon;

use crate::state::checkout_state::CheckoutState;

#[component]
pub fn HeroSection(
) -> impl IntoView {
    let (is_hamburger_open, set_hamburger_open) = create_signal(false);

    let toggle_menu = move || {
        set_hamburger_open.update(|open| *open = !*open);
    };
    let hamburger_menu_class = create_memo(move |_| {
        if is_hamburger_open.get() {
            "flex flex-col gap-6 items-center p-6 text-black uppercase bg-white"
        } else {
            "hidden" // Use hidden class when the menu is closed
        }
    });
    let checkout_state = CheckoutState::get();

    let start_time = move ||  checkout_state.pickup_date_formatted.get();
    let end_time = move || checkout_state.return_date_formatted.get();

    let min_start_date = create_rw_signal( DateTime::parse_from_str(&Local::now().to_string(), "%Y-%m-%d %H:%M:%S%.f %:z").map_or("".to_string(), |f| f.format("%Y-%m-%dT%H:%M").to_string()) );


    
    let (get_pickup_time_value, set_pickup_time_value) = create_signal(String::new());
    let (_, set_return_time_value) = create_signal(String::new());
    view! { 
        <div class="relative bg-gray-800 h-screen w-full text-white"
        style="background-image: url('/public/img/fuel-home.jpeg'); background-size: cover;;"
        >
            <div class="absolute inset-0 bg-black opacity-50"></div>

            // Navigation Bar
            <div class="flex relative flex-row hidden md:flex">
                <img src="/public/img/fueldao.svg" alt="FuelDAO Logo" class="h-10 p-2 basis-1/4" />
                <img src="/public/img/header.svg"  class="basis-1/2 hidden  md:flex" />
                <img src="/public/icons/user.svg"  class="hidden" />
            </div>
            <div class="flex absolute inset-x-0 top-0 z-20 justify-center items-center md:hidden">
                <div class="relative w-full md:bg-white max-w-[756.75px] md:rounded-b-[75px]">
                    <div class="flex justify-between items-center md:hidden">
                        <img src="/public/img/fueldao.svg" alt="FuelDAO Logo" class="h-10 p-2 basis-1/4" />

                        <button
                            id="menu-btn"
                            class="focus:outline-none"
                            on:click=move |_| toggle_menu()
                        >
                            <svg
                                class="w-8 h-8 text-black"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                viewBox="0 0 24 24"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M4 6h16M4 12h16M4 18h16"
                                ></path>
                            </svg>

                        </button>
                    </div>
                    // <!-- Hidden vertical menu for small devices (shown when hamburger clicked) -->

                    <div class=hamburger_menu_class>
                        <a href="#" class="block">
                            List your property
                        </a>
                        <a href="#" class="block">
                            Support
                        </a>
                        <a href="#" class="block">
                            Trips
                        </a>
                        <a href="#" class="block">
                            Sign in
                        </a>
                    </div>

                </div>
            </div>
            <div class="flex relative z-10 items-center p-4 lg:p-2 mx-auto max-w-screen-lg">
                <div class="p-6" style="margin-bottom: 70px;">
                    <h1 class="text-3xl font-bold lg:text-7xl">Explore your place <br />to stay</h1>
                    <div class="relative mt-2">
                        <div class="flex flex-col lg:flex-row items-center space-x-[10px] w-4/5 md:w-full  lg:w-[1074px] h-full lg:h-[106px] p-12 pt-4  lg:p-[27px_34px] bg-[#1D1D1D9C] backdrop-blur-[3px] border-t-[1px] border-l-[1px] border-gray-400 rounded-[22.5px] ">
                            <div class="flex flex-col gap-4 px-2 lg:flex-row lg:gap-[21px] lg:w-[1005.5px]">
                                // first field
                                <div class="bg-[#1D1D1D9C] flex   items-center bg-opacity-0 w-[238.75px] h-[52.5px] lg:p-[13px_12px] gap-[10px] rounded-[9px]">
                                    <button class="pt-4 pb-4 pl-2">
                                        <Icon
                                            class="rounded-full w-[24px] h-[24px]"
                                            icon=icondata::BiSearchRegular
                                        />
                                    </button>

                                    <input
                                        type="text"
                                        placeholder="Add your location"
                                        value= "Bengaluru"
                                        class="w-[213.81px] h-[24px]  pt-4 pr-4 pb-4 pl-0  bg-[#252525] bg-opacity-0 text-white placeholder-white"
                                        list="cities"
                                    />
                                    <datalist id="cities">
                                        <option value="Bengaluru"></option>
                                    </datalist>
                                </div>
                                // second field
                                <div class="flex items-center bg-[#1D1D1D9C] bg-opacity-0 w-[281px] h-[52px] p-[0px_11px] gap-[6px] rounded-tl-[9px] rounded-[9px] ">
                                            <input
                                                type="datetime-local"
                                                placeholder="Pickup datetime"
                                                class="bg-[#252525] pl-2  bg-opacity-0 text-white w-full placeholder-white"
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
                                // third field
                                <div class="flex items-center bg-[#1D1D1D9C] bg-opacity-0 w-[281px] h-[52px] p-[0px_11px] gap-[2px] rounded-[9px]">
                                            <input
                                                type="datetime-local"
                                                placeholder="Time"
                                                class="bg-[#252525] pl-2  bg-opacity-0 text-white w-full placeholder-white"
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
                            </div>
                            <a href="/search" class="flex justify-center items-center py-3 px-8 mt-8 w-full font-semibold text-white bg-green-600 rounded-md lg:mt-0 lg:w-auto hover:bg-green-700">
                                Search
                            </a>
                        // <button class="flex justify-center items-center bg-green-600 hover:bg-green-700 w-[141.75px] h-[52.5px] p-[19px_39px] gap-[10px] rounded-[9px]">
                        // <span class="font-semibold text-white text-[15px] leading-[13.98px] tracking-[0.075em] font-poppins">
                        // Search
                        // </span>
                        // </button>
                        </div>
                    </div>

                    <div class="absolute right-4 p-2 shadow-lg opacity-100 transform rotate-0 lg:p-4 lg:mr-10 lg:top-[464px] lg:w-[431.3px] lg:h-[185px]">
                        <p class="relative pl-4 font-bold text-left text-white font-poppins tracking-[0.075px] before:content-[ lg:text-[27px] lg:leading-[35.1px]''] before:absolute before:top-0 before:left-0 before:w-[4px] before:h-full before:bg-white">
                            "Premium Electric SUV voted ‘Best Family Electric Car’, at your fingertips."
                        </p>

                        <p class="p-4">Quality service with FuelDAO, guaranteed</p>
                    </div>
                </div>
            </div>

        </div>
    }
}


