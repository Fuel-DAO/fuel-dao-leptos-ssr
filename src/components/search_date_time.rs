use leptos::*;

use crate::components::SearchBar;

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

    view! { 
        <div class="h-80 flex flex-col justify-between  bg-gray-800 h-screen/2 w-full text-white bg-cover bg-center"
        style="background-image: url('/public/img/fuel-home.jpeg')"
        >
            // Navigation Bar
            <div class="flex  flex-row justify-between p-2 ">
                <a href="/"> <img src="/public/img/app.svg" alt="FuelDAO Logo" class="h-10 p-2 basis-1/4" /></a>
                <img src="/public/icons/user.svg"  class="hidden" />
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
            <SearchBar is_root=false/>

        </div>
    }
}
