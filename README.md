# Real-time strategy

## Buildings design

Elements needed for a building:

- A button to spawn it:
    - Asset with sprite image
    - Position of the button
- 3D Model asset
- Building time?
- Animations:
    - building
    - operating
    - destroyed
    - level of erosion based on health?
- Health
- Size
- State:
    - Getting Placed
    - Building
    - Active
- Selected - Highglight

```rs
#[derive(Building)]
struct RomeBuildingUnitSpawn {

}
```

## How to add a new building?

1. Adding a new `BuildingType`
2. Adding all the assets
3. 