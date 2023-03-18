



//use std::fs::File;
//use std::io::prelude::*;

mod maze;

use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let mut maze = maze::maze_mod::Maze {
        vertical_weight: 0.6,
        ..Default::default()
    };

    //have a button to generate maze

    
    
    //have a button to solve maze

    //have a button to reset maze

    //have a button to set start point

    //have a button to set end point






    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
            <canvas>
            </canvas>
            <div>
            // button to set start point
            
            // button to set end point

            // button to generate maze
            // button to solve maze
            // button to reset maze




            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
