use bevy::prelude::*;
use bevy_html_ui_builder::HtmlCssUIBuilder;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_html_ui)
        .run();
}

fn setup_html_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera สำหรับ UI
    commands.spawn(Camera2dBundle::default());

    // HTML + CSS content
    let html_content = r#"
    <html>
        <head>
            <style>
                .container {
                    padding: 20px;
                    background-color: #f0f0f0;
                    width: 800px;
                    height: 600px;
                }
                
                .title {
                    font-size: 32px;
                    color: #333333;
                    margin-bottom: 20px;
                    text-align: center;
                }
                
                .button {
                    background-color: #007bff;
                    color: white;
                    padding: 12px 24px;
                    margin: 10px;
                    border-radius: 6px;
                    width: 200px;
                    height: 50px;
                    font-size: 16px;
                }
                
                .button:hover {
                    background-color: #0056b3;
                }
                
                .card {
                    background-color: white;
                    padding: 16px;
                    margin: 12px;
                    border-radius: 8px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                    width: 300px;
                }
                
                #main-content {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <div id="main-content">
                    <h1 class="title">ยินดีต้อนรับสู่แอปพลิเคชัน</h1>
                    <div class="card">
                        <p>นี่คือการ์ดตัวอย่าง</p>
                        <button class="button">คลิกที่นี่</button>
                    </div>
                    <div class="card">
                        <p>การ์ดที่สอง</p>
                        <button class="button">ปุ่มอื่น</button>
                    </div>
                </div>
            </div>
        </body>
    </html>
    "#;

    // Parse และสร้าง UI
    let ui_builder = HtmlCssUIBuilder::new();
    if let Ok(elements) = ui_builder.parse_and_build(html_content) {
        ui_builder.spawn_bevy_ui(&mut commands, &asset_server, &elements);
    }
}
