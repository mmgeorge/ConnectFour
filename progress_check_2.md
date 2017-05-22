# Progress Check 2

- Added macro for declaring EntityTypes. Decided this route would allow for a much more efficient representation of entities as opposed to a HashMap. Handles boilerplate needed to expose metadata. For instance:
```
defEntity!(Dog(keys => id) {
   id: Int64,
   name: String,
   age: Int16
});
```
Declare a new EntityType Dog with the properties (id: Edm.Int64, name: Edm.String, and age: Edm.Int16). These get translated to the underlying Rust types to generate a struct Dog:
```
struct Dog {
  id: i64,
  name: String,
  age: i16
}
```
The reason for using a macro here, is that we also need to expose metadata info (`$metadata`). This is done by generating a `describe` function in the implementation of Dog that returns an `EntityDescr` that gets consumed by the `Model` class. 
```
fn describe() -> EntityDescr {
   EntityDescr {
      name: String::from(stringify!($name)),
      keys: vec![$(String::from(stringify!($key_name))),*],
      properties: vec![$(Property::new(
                           stringify!($key),
                           edm::from(stringify!($prop_type)))),*] 
    }
}
```
- Added macro for declaring EntitySets. Similar to above, handles boilerplate for generating metadata. For instance, the following code declares a new 'Dogs' EntitySet:
```
defEntitySet!(Dogs, Dog);
```
This generates a "descriptor" function which allows access to the EntityDescr for use by the Model, as well as provides a name function which, in this case, returns "Dogs." After this we can `impl EntitySet for Dogs` to define the CRUD operations we wish to expose for the set. 

- Added Model class for declaring models. This class stores the EntitySets associated with a model and handles generating the associated metadata. Models are built using the `ModelBuilder` helper class. 
```
 let dog_cafe_model: Model = ModelBuilder::new("dog_cafe")
    .add(Dogs::declare())
    .add(Customers::declare())
    .build();
```

- Added a Service class that acts as a wrapper around the Http Server library `Hyper`. It is built using the `ServiceBuilder` class using previously declared models. 
```
 let s: Service = ServiceBuilder::new("apis")
    .add(dog_cafe_model)
    .port(8080)
    .build();
 
 // Listen for connections
 s.start();  
```

This would then allow HTTP requests for myroot.xyz/apis/dog_cafe/. This is still in progress, but <...>/apis/dog_cafe/$metadata would return JSON-formatted CSDL for the available EntitySets (in this case Dogs and Customers), with a description of the allowable types and methods. The service supports JSON only (no XML), and as such we are using the [tenative JSON CSDL proposal](http://docs.oasis-open.org/odata/odata-json-csdl/v4.0/csprd01/odata-json-csdl-v4.0-csprd01.html).

Assuming we have implemented `read_list` for Dogs, then `GET myroot.xyz/apis/dog_cafe/Dogs` would drop us into the function we defined in our `impl EntitySet for Dogs`. Otherwise it returns the associated oData error. 

- Added Property Class (for EntityDescr and metadata)
- Added ServiceHandler class (to serve oData requests), still incomplete. 
- Added serde_json integration (for serialization/deserialization) 
- Added hyper integration (HTTP Server library)


Still TODO: 
- Finish up oData Service core functionality
- Connect Game to oData Service (likely using a SyncChannel)
- Finish up GUI




