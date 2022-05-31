# Rust CRUD Generator

This package contains a few macros which take as input a struct and, from it's definition, generate basic crud operations and api endpoints

Inspired by https://api-platform.com/, the objective is to be able to generate a basic CRUD API only through structure definition and configuration
Right now, the package only works with Diesel and Actix Web though this may be changed later on to offer more possibilities 

## Usage
You will need to place ``` rust #[derive(GenerateCrud)]``` over your struct definition and over other derive definitions if you have some. 
This allows the macro to take into account other macros you may derive, right now only the `Dummy` derive macro from [fake-rs](https://github.com/cksac/fake-rs) is taken into account but the macro will still expect a list of derived attributes, this may be changed. 
You may specify which fields are editable/visible by marking the field with the name of the operation `#[patchable]` for ex. (function does not fully work yet, only creatable and patchable work as expected)

## Current features
Currently, only the main macro "GenerateCrud" exists, which will generate the intermediate structs for inserting and updating, implement some basic crud functions for the given struct and generate Actix Web endpoints for said operations.
The update operation is not currently implemented but patch is which usually suffices.

Structs -> both New*StructName* and Patch*Structname* will be generated if said operations are requested, a *from* as well as a *from_multiple* functions are implemented, allowing easily to go from the base struct to the insertable one.

Functions ->
- create -> generates create function, taking a New*StructName*, create_multiple with a vector of New*StructName*, create_from which takes each field as param, creating the insertable struct itself
- read -> generates read function, taking an id and returns struct, get which returns multiple currently 50, needs to be changed with paging
- update -> not done yet
- patch -> patch function taking id and data as the Patch*Structname* generated previously
- delete -> delete function taking id 

Endpoints -> the endpoints are generated from the table name given, if none is given, it's the lowercase version of the struct name with an s at the end.
The endpoints use the given http method (get, post, patch, delete), a get to /*endpoint* will return multiple, a get to /*endpoint*/{id} wil return the one associated with the id.


## Todo
- real error handling -> very sparse information for user
- implement standard pagination
- add update operation
