use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div class="rounded-xl bg-gray-100 dark:bg-gray-800">
            <img class="w-full" src="https://placehold.co/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
            <div class="px-4 py-2">
                <div class="text-xl font-medium text-black dark:text-white">{ video.title.clone() }</div>
                <p class="text-gray-500 dark:text-gray-400">
                    { "Video description: Lorem ipsum dolor sit amet, consectetur adipisicing elit. Voluptatibus quia, nulla! Maiores et perferendis eaque, exercitationem praesentium nihil." }
                </p>
            </div>
        </div>
    }
}

#[function_component(VideosList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_click = on_click.clone();
    let list_elements = videos.iter().map(|video| {
        let on_video_select = {
            let on_click = on_click.clone();
            let video = video.clone();
            Callback::from(move |_| {
                on_click.emit(video.clone())
            })
        };

        html! {
            <div class="text-slate-800 flex w-full items-center rounded-md px-4 transition-all hover:bg-gray-100 dark:hover:bg-gray-800 dark:text-white">
                <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>     
            </div>
        }
    });

    html! {
        <div class="relative flex flex-col">
            <nav class="flex w-full flex-col gap-1">
                {for list_elements}
            </nav>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let videos = vec![
        Video {
            id: 1,
            title: "Building and breaking things".to_string(),
            speaker: "John Doe".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 2,
            title: "The development process".to_string(),
            speaker: "Jane Smith".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 3,
            title: "The Web 7.0".to_string(),
            speaker: "Matt Miller".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 4,
            title: "Mouseless development".to_string(),
            speaker: "Tom Jerry".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
    ];

    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
         <VideoDetails video={video.clone()} />
        }
    });

    html! {
        <>
            <div class="prose max-w-sm mx-auto py-6">
                <h1 class="dark:text-white">{ "WASM Test" }</h1>
                <h3 class="dark:text-white">{ "Click title for preview:" }</h3>
                <VideosList videos={videos} on_click={on_video_select.clone()} />
                // Add spacing between the videos list and the details
                <div class="h-4"></div>
                {for details}
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
