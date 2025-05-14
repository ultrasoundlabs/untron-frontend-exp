use leptos::prelude::*;

#[component]  
pub fn Home() -> impl IntoView {
    // Example state: In Next, you might use useState for form inputs.
    // In Leptos, use signals. For instance, a signal for an input amount:
    let (amount, set_amount) = signal(String::from("")); 
    
    view! {
        // Top-level container (if any) replicating the original DOM structure.
        <main class="flex flex-col items-center p-4">  // example classes
            // Example greeting and heading (mirroring "Good afternoon! Let's transfer now.")
            <h1 class="text-2xl font-bold mb-2">"Good afternoon!"</h1>
            <h2 class="text-xl mb-4">"Let's transfer now."</h2>
            
            // Example form section (You send / You receive fields)
            <div class="w-full max-w-md bg-gray-100 p-4 rounded">
                // "You send" field
                <label class="block text-sm font-medium mb-1">"You send"</label>
                <div class="flex items-center mb-3">
                    <input 
                        type="text" 
                        class="flex-1 border p-2 mr-2" 
                        prop:value=amount  // bind the input value to the signal
                        on:input:target=move |ev| { 
                            // On input, update the `amount` signal with the new value
                            set_amount.set(ev.target().value());
                        }
                    />
                    <img src="usdt-tron.png" alt="USDT Tron" class="w-6 h-6"/>  // token icon
                </div>
                
                // "You receive" field
                <label class="block text-sm font-medium mb-1">"You receive"</label>
                <div class="flex items-center mb-3">
                    <input type="text" class="flex-1 border p-2 mr-2" readonly=true prop:value=amount/>
                    <img src="usdt-arb.png" alt="USDT ARB" class="w-6 h-6"/>
                </div>
                
                // Destination address or ENS input
                <label class="block text-sm font-medium mb-1">"To"</label>
                <input type="text" placeholder="ENS or Address" class="w-full border p-2 mb-3"/>
                
                // "Paste" button or link (if present in Next UI, stubbed here)
                <button class="text-blue-600 text-sm underline mb-3"
                        on:click=move |_| {
                            // In a real app, here we'd paste from clipboard into the address field.
                        }>
                    "Paste"
                </button>
            </div>
            
            // FAQ Section (if any in Home)
            <section class="mt-8 w-full max-w-md">
                <h3 class="text-lg font-semibold mb-2">"FAQ"</h3>
                <ul class="list-disc list-inside text-sm text-gray-800">
                    { // We can render list items from an array of Q&A
                        let faqs = vec![
                            "What\'s USDT?", 
                            "What's Tron?", 
                            "What's Ethereum?", 
                            "What's Untron?", 
                            "How to send USDT from Tron?", 
                            "What about into Tron?", 
                            "How can I help?"
                        ];
                        faqs.into_iter().map(|q| view! {
                            <li>{q}</li>
                        }).collect::<Vec<_>>()  // collect into a Vec of views
                    }
                </ul>
            </section>
        </main>
    }
}