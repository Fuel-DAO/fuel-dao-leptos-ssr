use leptos::*;
use leptos_icons::Icon;

use crate::state::checkout_state::CheckoutState;

#[component]
pub fn HeroSectionNotHome(
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

    
    let (_, set_pickup_time_value) = create_signal(String::new());
    let (_, set_return_time_value) = create_signal(String::new());
    view! { 
        <div class="flex flex-col justify-between  bg-gray-800 h-screen/2 w-full text-white"
        style="background-image: url('/img/fuel-home.jpeg'); background-size: cover;"
        >
            // Navigation Bar
            <div class="flex  flex-row justify-between p-2 ">
                <img src="/img/fueldao.svg" alt="FuelDAO Logo" class="h-10 p-2 basis-1/4" />
                <img src="/icons/user.svg"  class="hidden inset-x-0 top-0 h-8 w-8 basis-1/4 md:flex" />
            </div>
            <div class="flex absolute inset-x-0 top-0 z-20 justify-center items-center md:hidden">
                <div class="relative w-full md:bg-white max-w-[756.75px] md:rounded-b-[75px]">
                    <div class="flex justify-end items-center md:hidden">
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
            <div class="flex flex-col justify-between z-10 items-center p-2">
                <div class="flex flex-col lg:flex-row items-center space-x-[10px]  md:w-full  lg:w-[1074px] h-full lg:h-[106px] p-6 pt-4  lg:p-[27px_34px] bg-[#1D1D1D9C] backdrop-blur-[3px] border-t-[1px] border-l-[1px] border-gray-400 rounded-[22.5px] ">
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
                                            />
                                    // <!-- First part of the input -->
                                    /* <div class="flex flex-row justify-between items-center w-[241.67px] gap-[2px] lg:h-[52.5px]">
                                        // <!-- First sub-field -->
                                        // <DateInputWithIcon
                                        // placeholder="Pickup Date"
                                        // icon=BsCalendar3
                                        // />
                                        //
                                        <div class="flex items-center w-[178.34px] lg:h-[22.34px] bg-[#252525] text-white placeholder-white lg:p-4 bg-opacity-0">
                                            <button type="datetime-local" class="pt-4 pb-4 pl-2 lg:pl-0">
                                                // on:click=move |ev: MouseEvent| {
                                                // ev.prevent_default();
                                                // if let Some(input) = input_ref.get() {
                                                // input.click();
                                                // }
                                                // }

                                                <Icon

                                                    class="w-[22px] h-[22px]"
                                                    icon=icondata::BsCalendar3
                                                />

                                            </button>

                                            <input
                                                type="date"
                                                placeholder="Pickup Date"
                                                class="bg-[#252525] pl-2 bg-opacity-0 text-white w-full placeholder-white"
                                                ref=pickup_ref_date
                                                on:input=move |ev| {
                                                    ev.prevent_default();
                                                    let value = event_target_value(&ev);
                                                    set_pickup_date_value.set(value);
                                                }
                                                value=pickup_date_value.get()
                                            />

                                        </div>

                                        // <!-- Separator -->
                                        <div class="border-white opacity-0 rotate-90 w-[2.5px] h-[0px] border-t-[1.5px]"></div>

                                        // <!-- Second sub-field -->
                                        <div class="flex items-center w-[90.34px] lg:h-[22.34px] bg-[#252525] text-white placeholder-white pt-4 pb-4 pl-0  bg-opacity-0">
                                            <button class="pt-4 pb-4">
                                                <Icon class="w-[24px] h-[24px]" icon=icondata::WiTime10 />
                                            </button>
                                            <input
                                                type="datetime-local"
                                                placeholder="Time"
                                                class="bg-[#252525] pl-2  bg-opacity-0 text-white w-full placeholder-white"
                                                ref=pickup_ref_time
                                                on:input=move |ev| {
                                                    ev.prevent_default();
                                                    let value = event_target_value(&ev);
                                                    set_pickup_time_value.set(value);
                                                }
                                                value=pickup_time_value.get()
                                            />
                                        </div>
                                    </div> */
                                </div>
                                // third field
                                <div class="flex items-center bg-[#1D1D1D9C] bg-opacity-0 w-[281px] h-[52px] p-[0px_11px] gap-[2px] rounded-[9px]">
                                            <input
                                                type="datetime-local"
                                                placeholder="Time"
                                                class="bg-[#252525] pl-2  bg-opacity-0 text-white w-full placeholder-white"

                                                on:input=move |ev| {
                                                    ev.prevent_default();
                                                    let value = event_target_value(&ev);
                                                    set_return_time_value.set(value.clone());
                                                    CheckoutState::set_return_date_value_formatted(value);
                                                }
                                                value=end_time.clone()
                                            />
                                    // <!-- Placeholder container -->
                                    /* <div class="flex justify-between items-center w-[240.67px] h-[52.5px] gap-[2px]">
                                        // <!-- First part of the placeholder -->
                                        <div class="flex items-center w-[178.34px] h-[22.34px] bg-[#252525] text-white  bg-opacity-0">
                                            <button class="pt-4 pb-4">
                                                <Icon
                                                    class="w-[22px] h-[22px]"
                                                    icon=icondata::BsCalendar3
                                                />
                                            </button>
                                            <input
                                                type="date"
                                                placeholder="Return Date"
                                                class="bg-[#252525] pl-4  bg-opacity-0 text-white w-full placeholder-white"
                                                ref=return_ref_date
                                                on:input=move |ev| {
                                                    ev.prevent_default();
                                                    let value = event_target_value(&ev);
                                                    set_return_date_value.set(value);
                                                }
                                                value=return_date_value.get()
                                            />

                                        </div>

                                        // <!-- Separator -->
                                        <div class="border-white opacity-0 rotate-90 w-[2.5px] h-[0px] border-t-[1.5px]"></div>

                                        // <!-- Second part of the placeholder -->
                                        <div class="flex items-center w-[98.34px] h-[22.34px] bg-[#252525] text-white  bg-opacity-0">
                                            <button class="pt-4 pb-4">
                                                <Icon class="w-[24px] h-[24px]" icon=icondata::WiTime10 />
                                            </button>
                                            <input
                                                type="time"
                                                placeholder="Time"
                                                class="bg-[#252525] pl-2  bg-opacity-0 text-white w-full placeholder-white"
                                                ref=return_ref_time

                                                on:input=move |ev| {
                                                    ev.prevent_default();
                                                    let value = event_target_value(&ev);
                                                    set_return_time_value.set(value);
                                                }
                                                value=return_time_value.get()
                                            />
                                            </div>
                                        
                                        </div> */
                                </div>
                            </div>
                            <a href="/search" class="flex justify-center items-center py-3 px-8 mt-8 w-full font-semibold text-white bg-green-600 rounded-md lg:mt-0 lg:w-auto hover:bg-green-700">
                                Search
                            </a>
                        
                        </div>
            </div>

        </div>
    }
}