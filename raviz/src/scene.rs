use crate::world_object::*;


pub struct WorldScene{
    objects: Vec<Box<dyn HittableObject>>
    //objects: Vec<&'a dyn HittableObject>
}


impl WorldScene{
    pub fn init() -> WorldScene{
        WorldScene {
            objects: Vec::new()
        }
    }

    pub fn addObject(&mut self, obj: impl HittableObject + 'static){
        self.objects.push(Box::new(obj));
    }

    pub fn objectCount(&self) -> usize{
        self.objects.len()
    }

    pub fn getVec(&self) -> &Vec<Box<dyn HittableObject>> {
        &self.objects
    }

    // pub fn getObject(&self, s: usize) -> Box<dyn HittableObject>{
    //     let ret = *self.objects.get(s).unwrap();
    //     ret
    // }
}