# Notes for developing the Nutrient Tracker

## Architecture

### Overall
```
            Backend                                          Frontend
                            |                      |
                            |                      |
             _________      |                      |  ______
            |         |     |                      | |      |     ______
 ______     |         |     |      Intranet        | | HTML |<-- |      |
|      |--->|  Rust   |     |  <---------------->  | | GUI  |--> | User |
| DB   |    |  Logic  |     |    RESTful HTTP      | |______|    |______|
|______|<---|         |     |                      |
            |_________|     |                      |
                            |                      |
                            |                      |
```

### Backend

```

                                _________
                               |         |            |
                               | Static  |  HTTP(S?)  |
                               | Content |----------->| Intranet
                               | Server  |            |
                               |_________|            |


 ______     ________________
|      |<--|                |
| DB   |   | DB abstraction |      TBC
|______|-->|________________|

```

## Backend


### Table Layout

1. Users
   - UID: unsigned int
   - Name: string
   - password hash: string
2. Nutrient Targets
   - user:             unsigned int (Users Table UID)
   - target nutrients: unsigned int (Nutrient Table UID)
   - date_begin:       date
   - date_end:         date // may not be needed if this can be quickly reconstructed by looking at the proceeding macro date begin
3. Units
   - UID: unsigned int
   - Name: string
   - abbreviation: string
4. Nutrients
   - UID: unsigned int
   - Calories:       float
   - Carbs (grams):  float
   - Protein(grams): float
   - Fat(grams):     float
   - Source:         string ("nutrient target"|"consumable") - for Liv debug only
5. Consumable
   - UID:            unsigned int
   - name:           string
   - notes:          string
   - portion amount: float
   - portion unit:   unsigned int (Units Table UID)
   - nutrients:      unsigned int (Nutrients Table UID)
6. Composite Consumable
   - UID:            unsigned int
   - name:           string
   - notes:          string
   - portion amount: float
   - portion unit:   unsigned int (Units Table UID)
7. Composite Consumable Nutrients
   - Composite Consumable ID: unsigned int (Composite Consumable Table UID)
      - used to refer to the composite consumable that these nutrients belong to
   - Consumable:              Option(unsigned int) (Consumable Table UID)
      - Used to refer to the nutrients that the owning consumabled comprises. Mutually exclusive with Consumable attribute.
   - Composite Consumable:    Option(unsigned int) (Composite Consumable Table UID)
      - Used to refer to the nutrients that the owning consumabled comprises. Mutually exclusive with Consumable attribute.
   - scaling:              float
8. Consumption Record
   - user:                 unsigned int (Users Table UID)
   - consumable:           Optional(unsigned int) (Consumable table id)
   - composite consumable: Optional(unsigned int) (Composite Consumable table id)
   - time: time
   - date: date // possibly a single time-date field depending on DB


### Requirements

- BE must serve static content to user using HTTP web interface
  - This includes HTML, JS and CSS content
- BE should serve static content over HTTPS
- BE must be able to store, edit, add and delete representations of following object types
  - Ingredient Item
  - Consumables
    - Meals
    - Drinks
  - Consumption instances
  - Users
- Additional Logging information should be stored
  - Interaction times, interations include
    - adding/removing/editing consumption instance/ingredients/consumables/composite consumables, macrosets
- External data should be importable via CSV



### API

- Definitions
  - object: [Targets, Users, Units, Nutrients, Consumables, CompositeConsumeable, Consumption Record ]
  - action: [add, edit, modify, update, delete]
  - summary: [sum,average]
  - timespan: [day,week,month]
  - granularity: [day,week,month]
- Endpoints
  - /
    - loads homepage
  - /api/action/{action}/{object}
  - /api/summary/{summary}/{timespan}/{granularity}



## Frontend


### Use case
  - Initial
    - Load page
    - Select user/login
    - Show homepage
   - adding a consumable
     - step a
     - step b
     - step c

### Layout
  - Homepage
    - Show nutrients and relation to targets
      - today
      - This week
      - Graphs for last X days?
  - Add
    - Add in a pre-made consumable
  - Manage
    - create items
      - Select item to create
      - Populate details
      - should this be hierarchical in presentation, then let backend handle everything or should every sub item be created before being incorperated
    - Update item
      - search by name?
      - pre-fill feilds
      - update
    - Delete item
      - search by name
      - select and delete
    - Personal
      - used to update personal params eg
        - weight
        - height
        - age
        - target nutrients
      - need some way to edit past nutrients?
    - Admin
      - God mode to edit/delete any record of any type


### Concept Layouts
Homepage
```
 ________________________________________________________________
|  Home  |  Add |  Manage  |  Personal  |       | <profile name> |
|----------------------------------------------------------------|
|                                                                |
|   | Today                                                      |
|   |             Carbs  Calories  Protein  Fat                  |
|   | Actual        a       b         c       d                  |
|   | Proportion    e       f         g       h                  |
|                                                                |
|                                                                |
|   |  Past 7 Days                                               |
|   |              Carbs  Calories  Protein  Fat                 |
|   |  Actual        a       b         c       d                 |
|   |  Proportion    e       f         g       h                 |
|                                                                |
|                                                                |
|   | Past Month                                                 |
|   |                                        x                   |
|   | 100% |              x                x                     |
|   |      |            x                x                       |
|   |      |          x      x    x  x x                         |
|   |      |        x                                            |
|   |      |      x            x                                 |
|   |      |    x                                                |
|   |   0% |______________________________________               |
|   |                                                            |
|________________________________________________________________|

```




# Assorted
- to run migration
```zsh
  export DATABASE_URL=postgres://nt_user:example_password@127.0.0.1:5432/nt_db && .cargo/bin/sea-orm-cli migrate up --verbose
```
