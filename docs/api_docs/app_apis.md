# APP CRUD APIs

## Add an App

> Add an app metadata by posting app data, where
> - `name`: descriptive name of the app
> - `short_name`: short name that is used in tracking cookie and instrumentation. Should be kept to max 3 characters.
>
> URL: `http://{{server-address}}/api/apps`

```
curl -X POST --location "http://127.0.0.1:6464/api/apps" \
    -H "Content-Type: application/json" \
    -d "{
          \"name\": \"Josh Stage App\",
          \"short_name\": \"jstg\"
        }"
```

## Update an App

> Update an app metadata by posting app data, where
> - `name`: descriptive name of the app
> - `short_name`: short name that is used in tracking cookie and instrumentation. Should be kept to max 3 characters.
>
> URL: `http://{{server-address}}/api/apps/{{app-id}}`

```
curl -X POST --location "http://127.0.0.1:6464/api/apps/app1" \
    -H "Content-Type: application/json" \
    -d "{
          \"name\": \"Josh App\",
          \"short_name\": \"josh\"
        }"
```

## View an App

> View an app metadata for a given `app-id`
>
> URL: `http://{{server-address}}/api/apps/{{app-id}}`

```
curl -X GET --location "http://127.0.0.1:6464/api/apps/app1" \
    -H "Accept: application/json"
```

## Get list of apps

> URL: `http://{{server-address}}/api/apps`

```
curl -X GET --location "http://127.0.0.1:6464/api/apps" \
    -H "Accept: application/json"
```