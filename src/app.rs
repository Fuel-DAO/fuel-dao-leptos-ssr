use crate::base_route::BaseRoute;
use crate::components::*;
use crate::error_template::{AppError, ErrorTemplate};
use crate::state::canisters::Canisters;
use crate::state::checkout_state::{CheckoutState, CheckoutUser};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
#[component]
//te
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_context(Canisters::default());
    provide_context(CheckoutState::default());
    provide_context(CheckoutUser::default());

    view! {
        <Stylesheet id="leptos" href="/pkg/fuel-dao-leptos-ssr.css" />

        // sets the document title
        <Title text="FuelDAO" />

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=BaseRoute>
                        <Route path="/" view=HomePage />
                        <Route path="/search" view=SearchResult />
                        <Route path="/checkout" view=CheckoutPage />
                    </Route>

                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Title text="FuelDao" />
        <main>
            <div class="flex flex-col min-h-screen">
                <HomeBanner is_root=true />
                // <Header/>
                // <HeroSection  />
                // <Search />
                <BestPlacedForTrips />
                
                // <Services/>
                <Advantages />
                <InvestInCar />
                <GetInTouch />
                // <StayInTheKnow/>
                <FooterSection />
                // <Footer />
            </div>
        </main>
    }
}
