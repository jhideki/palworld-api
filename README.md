# Palworld API

Query basic Pal stats and Pal breeding combinations

## Usage

To start using the API, follow these steps:
1. Sign up for RapidAPI.
2. Subscribe to the Palworld API.
3. Obtain your API key from RapidAPI.
4. Make requests to the desired endpoints.

## Endpoints

Please visit [RapidAPI]([https://rapidapi.com](https://rapidapi.com/philmckracken03/api/palworld-api1)) for usage.

## Authentication

The API uses API key authentication. To authenticate your requests, include your API key in the request headers.

## Rate Limits

There are no rate limits enforced by the API.

## Example Usage

Current parameters include the following:

### GET api/pals/Lamball
```json
{
  "data": {
    "id": 1,
    "image": "https://static.wikia.nocookie.net/palworld/images/0/01/Lamball_menu.png/",
    "pal": "Lamball",
    "wiki": "https://palworld.fandom.com/wiki/Lamball"
  },
  "status": "success"
}
```
### GET api/skills/Lamball
```json
{
  "data": {
    "id": 1,
    "pal": "Lamball",
    "skills": {
      "skill_0": {
        "desc": "Lamball's special skill. Curls into a ball, rolling after any enemies in its way. Becomes dizzy and unable to move after the attack ends.",
        "name": "roly_poly",
        "power": 35,
        "type": "neutral"
      },
      ...
    }
  },
  "status": "success"
}
```
### GET api/pals_info/Lamball
```json
{
  "data": {
    "genus": "humanoid",
    "id": 1,
    "name": "Lamball",
    "price": 1000,
    "rarity": 1,
    "size": "xs",
    "type_0": "neutral",
    "type_1": null
  },
  "status": "success"
}
```
### POST api/pals/breeding_calc
# Body
```json
{
"father": "Lamball",
 "mother": "Cattiva"
}
```
### GET api/pal_drops/Lamball
```json
{
  "data": {
    "drop 0": "wool",
    "drop 1": "lamball_mutton",
    "drop 2": null,
    "drop 3": null,
    "id": 1,
    "name": "Lamball"
  },
  "status": "success"
}
```
### GET api/pal_suitabilities/Lamball
```json
{
  "data": {
    "id": 1,
    "level_0": 1,
    "level_1": 1,
    "level_2": 1,
    "level_3": null,
    "level_4": null,
    "level_5": null,
    "level_6": null,
    "name": "Lamball",
    "suitability_0": "handiwork",
    "suitability_1": "transporting",
    "suitability_2": "farming",
    "suitability_3": null,
    "suitability_4": null,
    "suitability_5": null,
    "suitability_6": null
  },
  "status": "success"
}
```
## Contribution

Please follow these steps

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and ensure they are properly tested.
4. Commit your changes with descriptive commit messages.
5. Push your changes to your fork.
6. Submit a pull request to main


