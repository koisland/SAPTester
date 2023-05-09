# SAPTester
Front-end for [`saptest`](https://github.com/koisland/SuperAutoTest) Rust library.

## API
To view database fields, refer to the [`saptest` `db` module documentation](https://docs.rs/saptest/latest/saptest/db/index.html).

### **Pets**
*Get all pets.*
```bash
curl -X GET "https://saptest.fly.dev/db/pets"
```

*Get all pets from a specific pack.*
```bash
curl -X GET "https://saptest.fly.dev/db/pets?pack=Turtle"
```

*Get all tier 1 pets.*
```bash
curl -X GET "https://saptest.fly.dev/db/pets?tier=1"
```

*Get all tier 1 pets that have a faint trigger.*
```bash
curl -X GET "https://saptest.fly.dev/db/pets?tier=1&effect_trigger=Faint"
```

### **Foods**
*Get all pets.*
```bash
curl -X GET "https://saptest.fly.dev/db/foods"
```

*Get all foods from a specific pack.*
```bash
curl -X GET "https://saptest.fly.dev/db/foods?pack=Turtle"
```

*Get all tier 1 foods.*
```bash
curl -X GET "https://saptest.fly.dev/db/foods?tier=1"
```

*Get all tier 6 foods that have a random effect.*
```bash
curl -X GET "https://saptest.fly.dev/db/foods?tier=6&random=true"
```

### Battle
WIP


## Deployment
WIP.

```bash
cd frontend/
trunk build --release --public-url SAPTester
mv dist/ ../docs
```

## Sources
* https://www.w3schools.com/w3css/w3css_references.asp
* https://github.com/dxps/fullstack-rust-axum-dioxus-rwa/blob/main/backend/src/bin/server.rs
