use leptos::prelude::*;
use crate::components::prose::Prose;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="@container mx-auto flex flex-col  justify-between w-full max-w-5xl px-4 sm:px-6 lg:px-8 mb-10">
            <main class="prose text-lg dark:prose-invert">
                <article>
                    <div class="w-full md:flex md:items-center gap-16 mb-10">
                        <div class="text-pretty leading-9">
                            <h1 class="text-3xl font-bold mb-4">"Hi There 👋, Dhanush Kumar Here!"</h1>
                            <Prose>
                            <div class="text-lg">
                                <p>"I'm a passionate software developer from India, specializing in efficient and high-performance software development 
                                from scratch. I enjoy researching, building scalable APIs and web servers, and sharing my knowledge on" <a href="https://dev.to/dev-dhanushkumar" target="_blank">"dev.to"</a>  "and" 
                                <a href="https://stackoverflow.com/users/23688025/dev-dhanushkumar" target="_blank">"stack overflow"</a>. 
                                </p><br/>
                                <p>"Proficient in Rust, Java, Golang, and JavaScript, I focus on writing optimized and secure code that 
                                enhances system performance. I also have expertise in Docker, Kubernetes, and DevOps, enabling me to build scalable, secure, and optimized applications."
                                </p>  
                            </div>
                            </Prose>
                        </div>
                        <img src="/images/dev-dhanushkumar.jpg" alt="Dhanush Kumar" width={300}  class="rounded-3xl rotate-3 mx-4 md:mx-0"/>
                    </div>     
                    <div>
                        <p>
                            "I am a highly motivated and passionate full-stack developer with a strong foundation in multiple 
                            programming languages, including Rust, Golang, Java, Python, and JavaScript. My expertise spans 
                            backend development using frameworks like Spring Boot and Warp, as well as frontend technologies 
                            such as ReactJS. I am constantly exploring emerging technologies and enjoy tackling complex 
                            problems through innovative solutions."
                        </p><br/>

                        <p>
                            "Currently interning as a Software Development Engineer at Kriyaetive Verse Pvt. Ltd., 
                            I have gained hands-on experience in Rust and WebAssembly (WASM). I have developed web applications, 
                            implemented scalable authentication systems, and built real-time video communication platforms using 
                            WebRTC. My learning journey has been shaped by continuous research and development, allowing 
                            me to contribute effectively to team projects and explore new ways to enhance software performance."
                        </p><br/>

                        <p>
                            "My portfolio includes projects like a Leptos Dashboard, an advanced authentication module, and a 
                            WebRTC-based video chat application, all built using Rust and modern web technologies. Additionally, 
                            I have developed an AI-powered AR museum experience called AR-tifact, which integrates augmented 
                            reality and generative AI to enhance artifact exploration. My achievements include securing 2nd place 
                            in Hack@Train 2024 and Genesis 2024 hackathons, along with earning a Microsoft Certiport IT Specialist 
                            certification in Python."
                        </p><br/>

                        <p>
                            "I am currently pursuing a Bachelor’s degree in Computer Science and Engineering at Kongunadu College of 
                            Engineering and Technology, with a CGPA of 7.9. My technical expertise extends beyond programming to 
                            include tools and platforms such as Docker, Kubernetes, MySQL, PostgreSQL, and cloud computing. I am also 
                            deeply interested in security concepts like role-based authentication, making my applications not only 
                            efficient but also secure."
                        </p><br/>

                        <p>
                            "As a developer, I am driven by curiosity and a desire to build impactful, scalable, and user-centric applications. 
                            I aim to contribute to cutting-edge projects that push the boundaries of technology, particularly in cloud computing, 
                            AI-driven applications, and AR/VR experiences. My goal is to continuously evolve as a versatile developer and problem 
                            solver, ensuring that every project I work on brings meaningful value to users and businesses alike."
                        </p>
                    </div>
                </article>
            </main>
        </div>
    }
}