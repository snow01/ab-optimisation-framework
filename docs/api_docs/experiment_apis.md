# Experiment CRUD APIs

## Add an experiment

> URL: `http://{{server-address}}/api/experiments/{{app-id}}/{{project-id}}`

```
curl -X POST --location "http://127.0.0.1:6464/api/experiments/Ok-cUE-XSGUqyRWiqwKti/V1NauWW7Qs73MR5J6X0ZF" \
    -H "Content-Type: application/json" \
    -d "{
          \"name\": \"onboarding\",
          \"short_name\": \"onb\",
          \"version\": 0,
          \"kind\": \"Experiment\",
          \"inactive\": false,
          \"start_time\": null,
          \"end_time\": null,
          \"audiences\": [
            {
              \"kind\": \"Audience\",
              \"name\": \"new_users\",
              \"script_src\": \"ctx['new_user'] == True and ctx['app_version'] >= '4.7.3'\",
              \"size_kind\": \"Percent\",
              \"value\": 5
            }
          ],
          \"data\": {
            \"flow_config\": [
              \"s_i\",
              \"l_s\",
              \"c_s\",
              \"p_b_i\",
              \"p_p_i\",
              \"i_s\",
              \"f\"
            ]
          }
        }"
```

## Update an experiment

> URL: `http://{{server-address}}/api/experiments/{{app-id}}/{{project-id}}/{{experiment-id}}`

```
curl -X POST --location "http://127.0.0.1:6464/api/experiments/Ok-cUE-XSGUqyRWiqwKti/V1NauWW7Qs73MR5J6X0ZF/nL4sYPJP1BLBMvnON9tHG" \
    -H "Content-Type: application/json" \
    -d "{
          \"name\": \"onboarding\",
          \"short_name\": \"onb\",
          \"kind\": \"Experiment\",
          \"inactive\": false,
          \"start_time\": null,
          \"end_time\": null,
          \"frequency_constraint\": \"fns.allow_only_once(experiment)\",
          \"audiences\": [
            {
              \"kind\": \"Audience\",
              \"name\": \"new_users\",
              \"script_src\": \"ctx.get('new_user', False) and ctx.get('app_version', '0.0.0') >= '4.7.3'\",
              \"size_kind\": \"Percent\",
              \"value\": 100
            }
          ],
          \"data\": {
            \"flow_config\": [
              \"s_i\",
              \"l_s\",
              \"c_s\",
              \"p_b_i\",
              \"p_p_i\",
              \"i_s\",
              \"f\"
            ]
          }
        }"
```

## View an experiment

> URL: `http://{{server-address}}/api/experiments/{{app-id}}/{{project-id}}/{{experiment-id}}`

```
curl -X GET --location "http://127.0.0.1:6464/api/experiments/Ok-cUE-XSGUqyRWiqwKti/V1NauWW7Qs73MR5J6X0ZF/nL4sYPJP1BLBMvnON9tHG" \
    -H "Accept: application/json"
```

## List all experiments

> URL: `http://{{server-address}}/api/experiments/{{app-id}}/{{project-id}}`

```
curl -X GET --location "http://127.0.0.1:6464/api/experiments/Ok-cUE-XSGUqyRWiqwKti/V1NauWW7Qs73MR5J6X0ZF" \
    -H "Accept: application/json"
```


