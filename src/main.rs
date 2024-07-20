use  eframe::egui;

fn main() -> eframe::Result {
	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 249.0]),
		..Default::default()
	};
	eframe::run_native(
		"My egui APP",
		options, 
		Box::new(|cc| {
			egui_extras::install_image_loaders(&cc.egui_ctx);
			
			Ok(Box::<MyApp>::default())
		}),
	)
}

struct MyApp {
	name: String,
	age: u32,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			name: "Arthur".to_owned(),
			age:42,
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |u1| {
			u1.heading("My egui Application");
			u1.horizontal(|u1| {
				let name_label = u1.label("Your name: ");
				u1.text_edit_singleline(&mut self.name)
					.labelled_by(name_label.id);
			});
			u1.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
			if u1.button("Increment").clicked() {
					self.age += 1; 
				}
			u1.label(format!("Hello '{}' age {}", self.name, self.age));

		});
	}
}

