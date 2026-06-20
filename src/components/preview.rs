use leptos::prelude::*;

#[component]
pub fn StaticList() -> impl IntoView {
    let products = vec!["BlueBird", "Portal Suite", "f5a", "c7n"];

    view! {

        <h2>Static Products</h2>

        <ul>
            // perfectly fine for static data
            {
                products
                .into_iter()
                .map(|product| view! {
                    <li>{product}</li>
                })
                .collect::<Vec<_>>()
            }

            // [FAIL] products is moved
            // {
            //     products
            //     .into_iter()
            //     .map(|product| view! {
            //         <li>{product}</li>
            //     })
            //     .collect::<Vec<_>>()
            // }
        </ul>
    }
}

#[derive(Clone, Debug)]
struct ProductItem {
    id: usize,
    name: String,
}

#[component]
pub fn DynamicList() -> impl IntoView {
    let (products, products_mut) = signal::<Vec<ProductItem>>(vec![ProductItem {
        id: 1,
        name: "BlueBird".into(),
    }]);

    let on_add_product = move |_| {
        products_mut.update(|items| {
            let product_id = items.len() + 1;

            let product_name = format!("Product {}", product_id);

            items.insert(
                0,
                ProductItem {
                    id: product_id,
                    name: product_name,
                },
            );
        });
    };

    view! {
        <h2>Dynamic Products</h2>
        <button
            on:click=on_add_product
            class="border rounded px-2 py-1 bg-white hover:bg-slate-700 hover:text-white">
            "Add product"
        </button>



        <h3>Without For component</h3>
        // without move do not allow reactive
        <ul>
            {
               products
                .get()
                .into_iter()
                .map(|product| view! {
                    <li>{product.name}</li>
                })
                .collect::<Vec<_>>()
            }
        </ul>

        // using move allow reactive
        <ul>
            {
               move || products
                .get()
                .into_iter()
                .map(|product| view! {
                    <li>{product.name}</li>
                })
                .collect::<Vec<_>>()
            }
        </ul>

        <h3>With For component</h3>
        <ul>
            <For
                each=move || products.get()
                key=|product| product.id
                let(product)
                >
                <li>{product.id}: {product.name}</li>
            </For>
            <hr/>

            // another way with children
            <For
                each=move || products.get()
                key=|product| product.id
                children=move |product| {
                    view! {
                        <li>{product.id}: {product.name}</li>
                    }
                }
                >
            </For>

            <hr/>

            // another way
            <For
                each=move || products.get()
                key=|product| product.id
                let(ProductItem{id, name})
                >
                <li>{id}: {name}</li>
            </For>
        </ul>


    }
}
