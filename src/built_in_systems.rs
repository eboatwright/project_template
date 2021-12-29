use crate::util::clamp_f32;
use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;
use crate::util::delta_time;
use crate::Master;
use crate::built_in_components::*;
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use hecs::World;
use hecs::Entity;

// TODO: rigidbody entity collision?
pub fn rigidbody2d_update_system(world: &mut World, camera_pos: Vec2) {
	for (entity, (transform, rigidbody2d)) in &mut world.query::<(&mut Transform, &mut Rigidbody2D)>().without::<Static>() {
		rigidbody2d.grounded -= 1.0;

		rigidbody2d.velocity.x += rigidbody2d.gravity.x;
		rigidbody2d.velocity.x *= 1.0 - rigidbody2d.friction.x;
		transform.position.x += rigidbody2d.velocity.x;

		if let Ok(collider) = world.get::<BoxCollider2D>(entity) {
			for (_map_entity, map) in &mut world.query::<&Map>() {
				let mut tile_transform = Transform::default();
				let mut tile_collider;
				let mut collision = (true, true, true, true);
				let mut tile_pos = vec2(transform.position.y / map.tile_size as f32, transform.position.y / map.tile_size as f32 - 10.0);
				tile_pos.y = clamp_f32(0.0, tile_pos.y, map.tiles.len() as f32);
				tile_pos.x = clamp_f32(0.0, tile_pos.x, map.tiles[0].len() as f32);
				for y in (tile_pos.y - 10.0) as usize..(tile_pos.y - 10.0) as usize {
					for x in (tile_pos.x - 10.0) as usize..(tile_pos.x - 10.0) as usize {
						if map.tiles[y][x] != 0
						&& !map.ignore_collision.contains(&map.tiles[y][x]) {
							tile_transform.position = vec3(x as f32 * map.tile_size as f32, y as f32 * map.tile_size as f32, 0.0);
							tile_collider = match map.special_collisions.get(&map.tiles[y][x]) {
								Some((collider, up_collision, down_collision, left_collision, right_collision)) => {
									collision = (*up_collision, *down_collision, *left_collision, *right_collision);
									*collider
								},
								None => BoxCollider2D {
									size: vec2(map.tile_size as f32, map.tile_size as f32),
									offset: Vec2::ZERO,
								},
							};
							if collider.overlaps(transform, &tile_collider, &tile_transform) {
								if rigidbody2d.velocity.x < 0.0
								&& collision.2 {
									rigidbody2d.velocity.x = 0.0;
									transform.position.x = tile_transform.position.x + tile_collider.size.x - collider.offset.x + tile_collider.offset.x;
									if rigidbody2d.gravity.x < 0.0 {
										rigidbody2d.grounded = rigidbody2d.grounded_time;
									}
								}
								if rigidbody2d.velocity.x > 0.0
								&& collision.3 {
									rigidbody2d.velocity.x = 0.0;
									transform.position.x = tile_transform.position.x - collider.size.x - collider.offset.x + tile_collider.offset.x;
									if rigidbody2d.gravity.x > 0.0 {
										rigidbody2d.grounded = rigidbody2d.grounded_time;
									}
								}
							}
							collision = (true, true, true, true);
						}
					}
				}
			}
		}

		rigidbody2d.velocity.y += rigidbody2d.gravity.y;
		rigidbody2d.velocity.y *= 1.0 - rigidbody2d.friction.y;
		transform.position.y += rigidbody2d.velocity.y;

		if let Ok(collider) = world.get::<BoxCollider2D>(entity) {
			for (_map_entity, map) in &mut world.query::<&Map>() {
				let mut tile_transform = Transform::default();
				let mut tile_collider;
				let mut collision = (true, true, true, true);
				let mut tile_pos = vec2(transform.position.y / map.tile_size as f32, transform.position.y / map.tile_size as f32 - 10.0);
				tile_pos.y = clamp_f32(0.0, tile_pos.y, map.tiles.len() as f32);
				tile_pos.x = clamp_f32(0.0, tile_pos.x, map.tiles[0].len() as f32);
				for y in (tile_pos.y - 10.0) as usize..(tile_pos.y - 10.0) as usize {
					for x in (tile_pos.x - 10.0) as usize..(tile_pos.x - 10.0) as usize {
						if map.tiles[y][x] != 0
						&& !map.ignore_collision.contains(&map.tiles[y][x]) {
							tile_transform.position = vec3(x as f32 * map.tile_size as f32, y as f32 * map.tile_size as f32, 0.0);
							tile_collider = match map.special_collisions.get(&map.tiles[y][x]) {
								Some((collider, up_collision, down_collision, left_collision, right_collision)) => {
									collision = (*up_collision, *down_collision, *left_collision, *right_collision);
									*collider
								},
								None => BoxCollider2D {
									size: vec2(map.tile_size as f32, map.tile_size as f32),
									offset: Vec2::ZERO,
								},
							};
							if collider.overlaps(transform, &tile_collider, &tile_transform) {
								if rigidbody2d.velocity.y < 0.0
								&& collision.0 {
									rigidbody2d.velocity.y = 0.0;
									transform.position.y = tile_transform.position.y + tile_collider.size.y - collider.offset.y + tile_collider.offset.y;
									if rigidbody2d.gravity.y < 0.0 {
										rigidbody2d.grounded = rigidbody2d.grounded_time;
									}
								}
								if rigidbody2d.velocity.y > 0.0
								&& collision.1 {
									rigidbody2d.velocity.y = 0.0;
									transform.position.y = tile_transform.position.y - collider.size.y - collider.offset.y + tile_collider.offset.y;
									if rigidbody2d.gravity.y > 0.0 {
										rigidbody2d.grounded = rigidbody2d.grounded_time;
									}
								}
							}
							collision = (true, true, true, true);
						}
					}
				}
			}
		}
	}
}

pub fn animator_update_system(world: &mut World) {
	for (entity, animator) in &mut world.query::<&mut Animator>() {
		animator.animation_timer -= 1.0;
		if animator.animation_timer <= 0.0 {
			animator.animation_timer = animator.current_animation.frame_duration;
			animator.animation_frame_index += 1;
			if animator.animation_frame_index >= animator.current_animation.frames.len() {
				animator.animation_frame_index = 0;
				animator.dont_interrupt = false;
			}
			if let Ok(_animate_texture) = world.get::<AnimateTexture>(entity) {
				if let Ok(mut texture) = world.get_mut::<Texture>(entity) {
					texture.source.x = animator.current_frame() as f32 * texture.source.w;
				}
			}
		}
	}
}

pub fn particle_update_system(world: &mut World) {
	let mut to_spawn: Vec<(Transform, Rigidbody2D, Particle, Texture)> = Vec::new();
	for (_entity, (transform, particle_spawner)) in &mut world.query::<(&Transform, &mut ParticleSpawner)>() {
		particle_spawner.spawn_timer -= delta_time();
		if particle_spawner.spawn_timer <= 0.0 {
			particle_spawner.spawn_timer = particle_spawner.spawn_rate;
			to_spawn.push((
				Transform {
					position: vec3(0.0, 0.0, 0.0),
					scale: Vec3::ONE,
					rotation: Vec3::ZERO,
				},
				Rigidbody2D {
					velocity: vec2(gen_range(particle_spawner.min_velocity.x, particle_spawner.max_velocity.x), gen_range(particle_spawner.min_velocity.y, particle_spawner.max_velocity.y)),
					friction: particle_spawner.friction,
					gravity: particle_spawner.gravity,
					..Default::default()
				},
				Particle {
					life: particle_spawner.life,
				},
				Texture {
					texture: particle_spawner.texture,
					source: Rect {
						x: 0.0,
						y: 0.0,
						w: particle_spawner.texture.height(),
						h: particle_spawner.texture.height(),
					},
					..Default::default()
				},
			));
		}
	}
	world.spawn_batch(to_spawn);

	let mut to_destroy: Vec<Entity> = Vec::new();
	for (entity, (transform, particle)) in &mut world.query::<(&mut Transform, &mut Particle)>() {
		particle.life -= delta_time();
		if particle.life <= 0.0 {
			to_destroy.push(entity);
			continue;
		}
	}
	for entity in to_destroy {
		world.despawn(entity).unwrap();
	}
}

pub fn sin_wave_update_system(world: &mut World, master: &Master) {
	for (_entity, sin_wave) in &mut world.query::<&mut SinWave>() {
		sin_wave.value = f64::sin(master.time_since_start * sin_wave.speed + sin_wave.offset) * sin_wave.distance;
	}
}

pub fn camera_update_system(world: &mut World, master: &mut Master) {
	for (_entity, (transform, camera_target)) in &mut world.query::<(&Transform, &CameraTarget)>() {
		master.camera_pos = master.camera_pos.lerp((transform.position + camera_target.offset).truncate(), camera_target.smoothing).round();
	}

	for (_entity, (transform, follow_camera)) in &mut world.query::<(&mut Transform, &FollowCamera)>() {
		transform.position = (master.camera_pos.extend(0.0) + follow_camera.offset).round();
	}
}

pub fn follow_update_system(world: &mut World) {
	for (_entity, (follower_transform, follow)) in &mut world.query::<(&mut Transform, &Follow)>() {
		for (target_entity, target_transform) in &mut world.query::<&Transform>().without::<Follow>() {
			if follow.id == target_entity.id() {
				follower_transform.position = target_transform.position + follow.offset;
			}
		}
	}
}

pub fn texture_render_system(world: &mut World, camera_pos: Vec2, layer: &'static str) {
	for (entity, (transform, texture)) in &mut world.query::<(&Transform, &Texture)>() {
		if texture.render_layer == layer {
			let x_pos = transform.position.x - texture.source.w * transform.scale.x / 2.0 + texture.source.w / 2.0;
			let mut y_pos = transform.position.y - texture.source.h * transform.scale.y / 2.0 + texture.source.h / 2.0;
			if let Ok(sin_wave) = world.get::<SinWave>(entity) {
				y_pos += sin_wave.value as f32;
			}

			if x_pos + texture.source.w * transform.scale.x < camera_pos.x - SCREEN_WIDTH as f32 / 2.0
			|| x_pos > camera_pos.x + SCREEN_WIDTH as f32 / 2.0
			|| y_pos + texture.source.h * transform.scale.y < camera_pos.y - SCREEN_HEIGHT as f32 / 2.0
			|| y_pos > camera_pos.y + SCREEN_HEIGHT as f32 / 2.0 {
				continue;
			}

			draw_texture_ex(
				texture.texture,
				x_pos.round(),
				y_pos.round(),
				texture.color,
				DrawTextureParams {
					dest_size: Some(vec2(texture.source.w * transform.scale.x, texture.source.h * transform.scale.y)),
					source: Some(texture.source),
					rotation: transform.rotation.z,
					flip_x: false,
					flip_y: false,
					pivot: None,
				},
			);
		}
	}
}

pub fn map_render_system(world: &mut World, camera_pos: Vec2, layer: &'static str) {
	for (_entity, (map, texture)) in &mut world.query::<(&Map, &Texture)>() {
		if texture.render_layer == layer {
			for y in 0..map.tiles.len() {
				for x in 0..map.tiles[0].len() {
					if map.tiles[y][x] != 0
					&& (y as f32 + 1.0) * (map.tile_size as f32) > camera_pos.y - SCREEN_HEIGHT as f32 / 2.0
					&& (y as f32) * (map.tile_size as f32) < camera_pos.y + SCREEN_HEIGHT as f32 / 2.0
					&& (x as f32 + 1.0) * (map.tile_size as f32) > camera_pos.x - SCREEN_WIDTH as f32 / 2.0
					&& (x as f32) * (map.tile_size as f32) < camera_pos.x + SCREEN_WIDTH as f32 / 2.0 {
						draw_texture_ex(
							texture.texture,
							x as f32 * map.tile_size as f32,
							y as f32 * map.tile_size as f32,
							texture.color,
							DrawTextureParams {
								dest_size: Some(vec2(map.tile_size as f32, map.tile_size as f32)),
								source: Some(Rect {
									x: (map.tiles[y][x] - 1) as f32 * map.tile_size as f32,
									y: match map.y_offsets.get(&map.tiles[y][x]) {
										Some(y_offset) => *y_offset,
										None => 0.0,
									},
									w: map.tile_size as f32,
									h: map.tile_size as f32,
								}),
								rotation: 0.0,
								flip_x: false,
								flip_y: false,
								pivot: None,
							},
						);
					}
				}
			}
		}
	}
}

pub fn text_render_system(world: &mut World, layer: &'static str) {
	for (_entity, (transform, text)) in &mut world.query::<(&Transform, &TextRenderer)>() {
		if text.render_layer == layer {
			draw_text_ex(
				text.text,
				transform.position.x.round(),
				transform.position.y.round(),
				text.params,
			);
		}
	}
}