# mutation
```
mutation {
  addUser(name: "Filbert Franigan", kind: USER, friendIds: [1] ) {
    id
  } 
}
```

# Query
```
{
  users(name: "F") {
    name,
    id,
  }
}
```