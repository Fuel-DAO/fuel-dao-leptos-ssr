use leptos::{component, view, IntoView};

#[component]
pub fn Advantages() -> impl IntoView {
    view! {
        <section class="py-12 bg-white">
            <div class="container mx-auto text-center gap-50">
                // first sub div
                <div
                    class="flex flex-col gap-3 items-center h-[134px] lg:w-[1236.88px]"
                    style="margin-bottom: 49.99px;"
                >
                    // <!-- First Part -->
                    <div class="flex flex-col gap-2 px-2 h-auto max-w-[400px]">

                        <div class="flex flex-col gap-2 justify-center items-center">
                            <p class="font-bold text-left font-lato text-[20px] leading-[24px]">
                                ADVANTAGES
                            </p>
                        </div>
                        <div class="flex flex-col gap-2 pt-2">
                            <p class="font-medium text-left whitespace-nowrap font-lato text-[43px] leading-[51.59px]">
                                Why Choose Us?
                            </p>
                        </div>
                    </div>
                    // <!-- Second Part -->
                    <div class="flex flex-col gap-2 justify-center items-center p-2 pt-2 w-full h-[38px]">
                        <p class="text-left">
                        We promise to make this an unforgettable experience in your car rental journey. Some of our guarantees-
                        </p>
                    </div>
                </div>
                // second sub div
                // <h2 class="mb-8 text-3xl font-bold">"Why Choose Us ?"</h2>
                <div
                    class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-3"
                    style="margin-bottom: 79.99px;"
                >
                    <div class="flex flex-row gap-4 items-center w-full h-[154px]">
                        <div class="flex-shrink-0">
                            <img src="/public/icons/rocket.svg" alt="Easy Rent" class="w-20 h-20" />
                        </div>
                        <div class="flex flex-col">
                            <h3 class="text-lg font-bold text-left">Smooth Experience</h3>
                            <p class="text-left text-gray-700">
                            A rental experience focused on making your ride smooth- from booking to grooving!
                            </p>
                        </div>
                    </div>
                    <div class="flex gap-4 items-center w-full h-[154px]">
                        <div class="flex-shrink-0">
                            <img src="/public/icons/check.svg" alt="Premium Quality" class="w-20 h-20" />
                        </div>
                        <div class="flex flex-col">
                            <h3 class="text-lg font-bold text-left">Premium Quality</h3>
                            <p class="text-left text-gray-700">
                            Clean cars and regular battery health monitoring, to make sure you never stop
                            </p>
                        </div>
                    </div>
                    <div class="flex gap-4 items-center w-full h-[154px]">
                        <div class="flex-shrink-0">
                            <img
                                src="/public/icons/agent.svg"
                                alt="Professional Agent"
                                class="w-20 h-20"
                            />
                        </div>
                        <div class="flex flex-col">
                            <h3 class="text-lg font-bold text-left">24x7 Support</h3>
                            <p class="text-left text-gray-700">
                            Coldplay tickets may not have been available for you, but we always are on your trip!
                            </p>
                        </div>
                    </div>
                </div>
                <div
                    class="grid grid-cols-1 gap-8 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-3"
                    style="margin-bottom: 79.99px;"
                >
                    <div class="flex flex-row gap-4 items-center w-full h-[154px]">
                        <div class="flex-shrink-0">
                            <img src="/public/icons/car-safety.svg" alt="Easy Rent" class="w-20 h-20" />
                        </div>
                        <div class="flex flex-col">
                            <h3 class="text-lg font-bold text-left">Car Safety</h3>
                            <p class="text-left text-gray-700">
                            Our car components are checked regularly, so check this off your list and ride comfortably.
                            </p>
                        </div>
                    </div>
                    <div class="flex flex-row gap-4 items-center w-full h-[154px]">
                        <div class="flex-shrink-0">
                            <img src="/public/icons/refund.svg" alt="Premium Quality" class="w-20 h-20" />
                        </div>
                        <div class="flex flex-col">
                            <h3 class="text-lg font-bold text-left">Route Planning</h3>
                            <p class="text-left text-gray-700">
                            "We’ll help you plan all your stops to your destination: while your car eats, so should you!" 
                            </p>
                        </div>
                    </div>
                    <div class="flex flex-row gap-4 items-center w-full h-[154px]">
                        <div class="flex-shrink-0">
                            <img
                                src="/public/icons/live-monitoring.svg"
                                alt="Professional Agent"
                                class="w-20 h-20"
                            />
                        </div>
                        <div class="flex flex-col">
                            <h3 class="text-lg font-bold text-left">Live Monitoring</h3>
                            <p class="text-left text-gray-700">
                            Our trips are monitored as per government regulations. Your safety, our responsibility.
                            </p>
                        </div>
                    </div>
                </div>

            </div>
        </section>
    }
}
