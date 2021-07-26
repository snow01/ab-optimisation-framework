# Project CRUD APIs

## Add a Project

> Add or a project metadata by posting project data, where a project is defined by
>
> URL: `http://{{server-address}}/api/projects/{{app-id}}`

```
curl -X POST --location "http://127.0.0.1:6464/api/projects/Ok-cUE-XSGUqyRWiqwKti" \
    -H "Content-Type: application/json" \
    -d "{
          \"name\": \"Android\",
          \"short_name\": \"android\"
        }"
```


## Update a Project

> URL: `http://{{server-address}}/api/projects/{{app-id}}/{{project-id}}`

```
curl -X POST --location "http://127.0.0.1:6464/api/projects/Ok-cUE-XSGUqyRWiqwKti/V1NauWW7Qs73MR5J6X0ZF" \
    -H "Content-Type: application/json" \
    -d "{
          \"name\": \"Android\",
          \"short_name\": \"android\"
        }"
```

## View a Project

> View a project metadata for a given `app-id` and `project-id`
>
> URL: `http://{{server-address}}/api/projects/{{app-id}}/{{project-id}}`

```
curl -X GET --location "http://127.0.0.1:6464/api/projects/Ok-cUE-XSGUqyRWiqwKti/android" \
    -H "Accept: application/json"
```

## List all projects

> URL: `http://{{server-address}}/api/experiments/{{app-id}}`

```
curl -X GET --location "http://127.0.0.1:6464/api/projects/Ok-cUE-XSGUqyRWiqwKti" \
    -H "Accept: application/json"
```