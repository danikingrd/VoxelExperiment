use std::time::Duration;

use crate::{
    event::{Event, Events},
    resources::{DeltaTime, GameMode, TerrainMap},
};

pub struct State {
    world: apecs::World,
}

impl State {
    pub fn client() -> apecs::anyhow::Result<Self> {
        let state = Self::new(GameMode::Client)?;
        Ok(state)
    }

    pub fn server() -> apecs::anyhow::Result<Self> {
        let state = Self::new(GameMode::Server)?;
        Ok(state)
    }

    pub fn new(mode: GameMode) -> apecs::anyhow::Result<Self> {
        let mut world = apecs::World::default();
        world.with_default_resource::<DeltaTime>()?;
        world.with_default_resource::<TerrainMap>()?;
        world.with_resource(mode)?;
        Ok(Self { world })
    }

    pub fn tick(&mut self, dt: Duration) {
        self.resource_mut::<DeltaTime>().0 = dt.as_secs_f32();
        if let Err(e) = self.world.tick() {
            log::error!("{}", e);
        }
    }

    pub fn with_event<E: Event>(&mut self, name: &str) {
        match self.world.set_resource::<Events<E>>(Events::default()) {
            Ok(world) => {
                self.world
                    .with_system(name, super::event::event_update_system::<E>)
                    .unwrap();
            },
            Err(e) => log::error!("Failed to add event system for {}: {}", name, e),
        }
    }

    pub fn resource<R: apecs::IsResource>(&self) -> &R {
        self.world
            .resource::<R>()
            .expect("Tried to fetch an invalid resource")
    }

    pub fn resource_mut<R: apecs::IsResource>(&mut self) -> &mut R {
        self.world
            .resource_mut::<R>()
            .expect("Tried to fetch an invalid resource")
    }

    pub fn query<Q: apecs::IsQuery + 'static>(&mut self) -> apecs::QueryGuard<'_, Q> {
        self.world.query::<Q>()
    }

    pub fn ecs(&self) -> &apecs::World {
        &self.world
    }

    pub fn ecs_mut(&mut self) -> &mut apecs::World {
        &mut self.world
    }
}
