use dioxus::prelude::*;
use chrono::Datelike;

use crate::constants::*;


#[component]
pub fn Footer(children: Element) -> Element {
rsx! {
    div { class: "px-4 sm:px-6 lg:px-8 pb-2  bg-CustomBackground lg:grid-cols-2 gap-8",

        // Bottom Section with Social Media Links
        div { class: "text-white sm:flex sm:items-center sm:justify-between px-4 py-2 text-center bg-CustomNav shadow-md",
            span { class: "text-sm text-cent sm:text-center ",
                "© {chrono::Local::now().year()}"
                a { class: "hover:underline", href: "{WEBSITE_URL}", " {COMPANY_NAME}" }
                ". All Rights Reserved | "
            }


            div { class: "flex mt-4 space-x-6 sm:justify-center sm:mt-0",


                // X

                a {
                    class: " hover:text-gray-900 dark:hover:text-white",
                    href: "https://x.com/Dylan_Rayburn",
                    svg {
                        class: "w-5 h-5",
                        fill: "white",
                        view_box: "0 0 24 24",
                        path { d: "M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z" }
                    }
                }


                // Linkedin
                a {
                    class: " hover:text-gray-900 dark:hover:text-white",
                    href: "https://www.linkedin.com/in/dylan-rayburn-a6b93499/",
                    svg {
                        class: "w-5 h-5",
                        fill: "white",
                        view_box: "0 0 24 24",
                        path {
                            clip_rule: "evenodd",
                            d: "M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z",
                            fill_rule: "evenodd",
                        }
                    }
                }
            }
        }
    }
}}