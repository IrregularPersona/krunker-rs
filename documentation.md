# Developer APIs

The Developer APIs provide read-only access to public game data for third-party integrations.

## Authentication

All developer API endpoints require the `X-Developer-API-Key` header.

```
X-Developer-API-Key: <your-api-key>
```

**Error Response (403 Forbidden):**
```json
{
  "error": "Not allowed."
}
```

## Rate Limiting

All endpoints are rate-limited to **60 requests per minute** per API key.

**Rate Limit Headers:**

Every response includes headers to help you track your usage:

| Header | Description |
|--------|-------------|
| `X-RateLimit-Limit` | Maximum requests allowed per window (60) |
| `X-RateLimit-Remaining` | Requests remaining in the current window |
| `X-RateLimit-Reset` | Unix timestamp when the rate limit resets |

**Rate Limit Exceeded Response (429 Too Many Requests):**
```json
{
  "error": "Rate limit exceeded",
  "retry_after": 45
}
```

| Field | Type | Description |
|-------|------|-------------|
| error | string | Error message |
| retry_after | int | Seconds until the rate limit resets |

---

## Player Endpoints

### Get Player Profile

Retrieve comprehensive player statistics and profile information.

```
GET /api/player/:playername
```

**Parameters:**
| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| playername | string | path | yes | The player's in-game username |

**Response:**
```json
{
  "player_name": "string",
  "clan": "string",
  "verified": true,
  "flag": 5,
  "badges": [1, 2, 3],
  "following": 50,
  "followers": 100,
  "ranked": [
    {
      "region": 0,
      "mmr": 2250,
      "wins": 45,
      "losses": 30,
      "kills": 800,
      "deaths": 500,
      "assists": 200,
      "score": 15000,
      "damage_done": 120000,
      "time_played": 36000
    }
  ],
  "kr": 1000,
  "level": 50,
  "junk": 1500.5,
  "inventory": 25000,
  "score": 1000000,
  "spk": 125.5,
  "kills": 8000,
  "deaths": 4000,
  "kdr": 2.0,
  "kpg": 10.5,
  "games": 760,
  "wins": 400,
  "losses": 360,
  "assists": 1200,
  "melees": 150,
  "beatdowns": 50,
  "bullseyes": 30,
  "headshots": 3000,
  "legshots": 500,
  "wallbangs": 200,
  "shots": 100000,
  "hits": 45000,
  "misses": 55000,
  "time_played": 360000,
  "nukes": 25,
  "airdrops": 150,
  "airdrops_stolen": 30,
  "slimes": 40,
  "juggernauts": 20,
  "juggernauts_killed": 15,
  "warmachines": 10,
  "hacker_tagged": false,
  "created_at": "2021-05-10T14:30:00Z"
}
```

**Response Fields:**
| Field | Type | Description |
|-------|------|-------------|
| player_name | string | Player's username |
| clan | string | Player's clan name (empty string if not in a clan) |
| verified | bool | Whether the player is verified |
| flag | int | Player's country flag index |
| badges | int[] | Array of badge IDs the player has earned |
| following | int | Number of players this player is following |
| followers | int | Number of players following this player |
| ranked | object[] | Array of ranked profiles for the current season, one per region (only includes regions where player has completed 6+ placement matches) |
| ranked[].region | int | Region ID |
| ranked[].mmr | int | Matchmaking rating |
| ranked[].wins | int | Ranked wins in this region |
| ranked[].losses | int | Ranked losses in this region |
| ranked[].kills | int | Total kills in ranked matches |
| ranked[].deaths | int | Total deaths in ranked matches |
| ranked[].assists | int | Total assists in ranked matches |
| ranked[].score | int | Total score in ranked matches |
| ranked[].damage_done | int | Total damage dealt in ranked matches |
| ranked[].time_played | int | Total time played in ranked matches (seconds) |
| kr | int | Currency balance |
| level | int | Calculated from score (0.03 × √score, min 1) |
| junk | float | ELO/Junk rating |
| inventory | int | Total skin value |
| score | int | Total score |
| spk | float | Score per kill |
| kills | int | Total kills |
| deaths | int | Total deaths |
| kdr | float | Kill/Death ratio |
| kpg | float | Kills per game |
| games | int | Games played |
| wins | int | Games won |
| losses | int | Games lost (games - wins) |
| assists | int | Total assists |
| melees | int | Melee kills |
| beatdowns | int | Fist kills |
| bullseyes | int | Thrown weapon kills |
| headshots | int | Headshot kills |
| legshots | int | Leg shot kills |
| wallbangs | int | Wallbang kills |
| shots | int | Total shots fired |
| hits | int | Shots that connected |
| misses | int | Shots missed (shots - hits) |
| time_played | int64 | Seconds played |
| nukes | int | Nukes called in |
| airdrops | int | Airdrops called in |
| airdrops_stolen | int | Airdrops stolen from enemies |
| slimes | int | Slimes called in |
| juggernauts | int | Juggernaut killstreaks earned |
| juggernauts_killed | int | Enemy juggernauts killed |
| warmachines | int | Warmachines called in |
| hacker_tagged | bool | Whether the player has been flagged as a cheater |
| created_at | string | Account creation date (RFC3339 format) |

---

### Get Player Inventory

Retrieve a player's skin inventory.

```
GET /api/player/:playername/inventory
```

**Parameters:**
| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| playername | string | path | yes | The player's in-game username |

**Response:**
```json
[
  {
    "skin_index": 123,
    "count": 5
  }
]
```

**Response Fields:**
| Field | Type | Description |
|-------|------|-------------|
| skin_index | int | Identifier for the skin |
| count | int | Number owned (excludes market listings) |

---

### Get Player Match History

Retrieve a player's ranked match history (paginated).

```
GET /api/player/:playername/matches
```

**Parameters:**
| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| playername | string | path | yes | The player's in-game username |
| page | int | query | no | Page number (default: 1) |
| season | int | query | no | Season filter (default: all seasons) |

**Response:**
```json
{
  "page": 1,
  "per_page": 10,
  "matches": [
    {
      "match_id": 123456789,
      "date": "2024-01-15T10:30:00Z",
      "map": 5,
      "duration": 600,
      "season": 12,
      "region": 1,
      "kills": 15,
      "deaths": 8,
      "assists": 3,
      "score": 1500,
      "damage_done": 2500,
      "headshots": 5,
      "accuracy": 35,
      "objective_score": 200,
      "kr": 50,
      "victory": 1,
      "rounds_won": 6,
      "team": 1,
      "play_time": 580
    }
  ]
}
```

**Notes:**
- Only ranked matches are returned
- Page size is fixed at 10 records
- Matches are ordered by date descending

---

### Get Player Posts

Retrieve a player's social media posts (paginated).

```
GET /api/player/:playername/posts
```

**Parameters:**
| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| playername | string | path | yes | The player's in-game username |
| page | int | query | no | Page number (default: 1) |

**Response:**
```json
{
  "page": 1,
  "per_page": 10,
  "posts": [
    {
      "date": "2024-01-15T10:30:00Z",
      "text": "Just hit level 100!",
      "votes": 42,
      "comment_count": 5
    }
  ]
}
```

---

## Match Endpoints

### Get Match Details

Retrieve detailed information about a specific ranked match.

```
GET /api/match/:id
```

**Parameters:**
| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| id | int64 | path | yes | The match ID |

**Response:**
```json
{
  "match_id": 123456789,
  "date": "2024-01-15T10:30:00Z",
  "map": 5,
  "duration": 600,
  "season": 12,
  "region": 1,
  "participants": [
    {
      "player_name": "Player1",
      "kills": 15,
      "deaths": 8,
      "assists": 3,
      "score": 1500,
      "damage_done": 2500,
      "headshots": 5,
      "accuracy": 35,
      "objective_score": 200,
      "victory": 1,
      "rounds_won": 6,
      "team": 1,
      "play_time": 580
    }
  ]
}
```

**Notes:**
- Only ranked matches are returned
- Participants are ordered by team, then by score descending

---

## Clan Endpoints

### Get Clan Profile

Retrieve a clan's public information.

```
GET /api/clan/:clanname
```

**Parameters:**
| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| clanname | string | path | yes | The clan's name |

**Response:**
```json
{
  "name": "Elite Squad",
  "owner_name": "ClanLeader",
  "score": 5000000,
  "rank": 15,
  "member_count": 25,
  "created_at": "2023-06-01T00:00:00Z",
  "discord": "abc123xyz"
}
```

**Response Fields:**
| Field | Type | Description |
|-------|------|-------------|
| name | string | Clan name |
| owner_name | string | Name of the clan owner |
| score | int64 | Clan's total score |
| rank | int | Clan's leaderboard rank |
| member_count | int | Number of members in the clan |
| created_at | string | Clan creation date |
| discord | string | Discord invite code (empty string if not set) |

---

### Get Clan Members

Retrieve a clan's member list (paginated).

```
GET /api/clan/:clanname/members
```

**Parameters:**
| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| clanname | string | path | yes | The clan's name |
| page | int | query | no | Page number (default: 1) |

**Response:**
```json
{
  "page": 1,
  "per_page": 10,
  "members": [
    {
      "player_name": "ClanLeader",
      "role": 3
    },
    {
      "player_name": "Member1",
      "role": 1
    }
  ]
}
```

**Notes:**
- Page size is fixed at 10 records
- Members are sorted by role (descending), then by score (descending)

---

## Leaderboard Endpoints

### Get Ranked Leaderboard

Retrieve the ranked leaderboard for a specific region.

```
GET /api/leaderboard/:region
```

**Parameters:**
| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| region | int | path | yes | The region ID (2 = Asia, 3 = Europe, 4 = North America) |
| page | int | query | no | Page number (default: 1) |

**Response:**
```json
{
  "page": 1,
  "per_page": 10,
  "season": 5,
  "region": 0,
  "entries": [
    {
      "position": 1,
      "player_name": "TopPlayer",
      "mmr": 2500,
      "wins": 150,
      "losses": 50,
      "kills": 3000,
      "deaths": 1500,
      "assists": 800,
      "score": 50000,
      "damage_done": 450000
    }
  ]
}
```

**Response Fields:**
| Field | Type | Description |
|-------|------|-------------|
| page | int | Current page number |
| per_page | int | Number of entries per page (10) |
| season | int | Current ranked season |
| region | int | Region ID |
| entries | object[] | Array of leaderboard entries |
| entries[].position | int | Leaderboard position |
| entries[].player_name | string | Player's username |
| entries[].mmr | int | Matchmaking rating |
| entries[].wins | int | Total ranked wins |
| entries[].losses | int | Total ranked losses |
| entries[].kills | int | Total kills in ranked matches |
| entries[].deaths | int | Total deaths in ranked matches |
| entries[].assists | int | Total assists in ranked matches |
| entries[].score | int | Total score in ranked matches |
| entries[].damage_done | int | Total damage dealt in ranked matches |

**Notes:**
- Only players with 6+ completed placement matches appear on the leaderboard
- Page size is fixed at 10 records
- Entries are sorted by MMR descending

---

## Map Endpoints

### Get Map by Name

Retrieve map information by its name.

```
GET /api/map/:mapname
```

**Parameters:**
| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| mapname | string | path | yes | The map's name (URL encoded) |

**Response:**
```json
{
  "map_id": 12345,
  "name": "MyAwesomeMap",
  "description": "A fun parkour map",
  "creator_name": "MapMaker",
  "votes": 500,
  "gameplays": 10000,
  "playtime": 3600000,
  "category": 5,
  "created_at": "2023-01-15T10:30:00Z",
  "updated_at": "2023-06-20T14:00:00Z",
  "leaderboard_type": "time",
  "leaderboard_order": 0
}
```

**Response Fields:**
| Field | Type | Description |
|-------|------|-------------|
| map_id | int | Unique map identifier |
| name | string | Map name |
| description | string | Map description |
| creator_name | string | Username of the map creator |
| votes | int | Total upvotes |
| gameplays | int | Number of times the map has been played |
| playtime | int64 | Total playtime in milliseconds |
| category | int | Map category ID |
| created_at | string | When the map was created |
| updated_at | string | When the map was last updated |
| leaderboard_type | string | Type of leaderboard (e.g., "time", "score", empty if none) |
| leaderboard_order | int | Sort order for leaderboard (0 = ascending/lower is better, 1 = descending/higher is better) |

**Notes:**
- Returns 404 if map not found or is restricted/deleted
- Map name is case-sensitive

---

### Get Map Leaderboard

Retrieve the leaderboard for a specific map (paginated).

```
GET /api/map/:mapname/leaderboard
```

**Parameters:**
| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| mapname | string | path | yes | The map's name (URL encoded) |
| page | int | query | no | Page number (default: 1) |

**Response:**
```json
{
  "page": 1,
  "per_page": 25,
  "map_name": "MyAwesomeMap",
  "leaderboard_type": "time",
  "leaderboard_order": 0,
  "entries": [
    {
      "position": 1,
      "player_name": "SpeedRunner",
      "value": 12345,
      "date": "2024-01-15T10:30:00Z"
    }
  ]
}
```

**Response Fields:**
| Field | Type | Description |
|-------|------|-------------|
| page | int | Current page number |
| per_page | int | Number of entries per page (25) |
| map_name | string | The map name queried |
| leaderboard_type | string | Type of leaderboard (e.g., "time", "score") |
| leaderboard_order | int | Sort order (0 = ascending, 1 = descending) |
| entries | object[] | Array of leaderboard entries |
| entries[].position | int | Leaderboard position |
| entries[].player_name | string | Player's username |
| entries[].value | int | Score or time value |
| entries[].date | string | When the entry was recorded |

**Notes:**
- Returns empty entries array if the map has no leaderboard configured
- Page size is fixed at 25 records
- Entries are sorted by value according to leaderboard_order

---

## Mod Endpoints

### Get Mods

Retrieve a paginated list of mods, sorted by votes (most popular first).

```
GET /api/mods
```

**Parameters:**
| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| page | int | query | no | Page number (default: 1) |

**Response:**
```json
{
  "page": 1,
  "per_page": 10,
  "mods": [
    {
      "mod_id": 12345,
      "name": "AwesomeMod",
      "description": "A cool mod that changes textures",
      "creator_name": "ModCreator",
      "votes": 5000,
      "featured": true,
      "version": 15,
      "created_at": "2023-01-15T10:30:00Z",
      "updated_at": "2024-06-20T14:00:00Z"
    }
  ]
}
```

**Response Fields:**
| Field | Type | Description |
|-------|------|-------------|
| page | int | Current page number |
| per_page | int | Number of mods per page (10) |
| mods | object[] | Array of mod objects |
| mods[].mod_id | int | Unique mod identifier |
| mods[].name | string | Mod name |
| mods[].description | string | Mod description |
| mods[].creator_name | string | Username of the mod creator |
| mods[].votes | int | Number of upvotes |
| mods[].featured | bool | Whether the mod is featured |
| mods[].version | int | Mod version number |
| mods[].created_at | string | When the mod was first published |
| mods[].updated_at | string | When the mod was last updated |

**Notes:**
- Mods are sorted by votes (most popular first)
- Only active mods are returned (deleted mods are excluded)
- Mods from banned users are excluded

---

### Get Mod by Name

Retrieve detailed information about a specific mod.

```
GET /api/mods/:modname
```

**Parameters:**
| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| modname | string | path | yes | The mod's name (URL encoded) |

**Response:**
```json
{
  "mod_id": 12345,
  "name": "AwesomeMod",
  "description": "A cool mod that changes textures",
  "creator_name": "ModCreator",
  "votes": 5000,
  "featured": true,
  "version": 15,
  "created_at": "2023-01-15T10:30:00Z",
  "updated_at": "2024-06-20T14:00:00Z"
}
```

**Response Fields:**
| Field | Type | Description |
|-------|------|-------------|
| mod_id | int | Unique mod identifier |
| name | string | Mod name |
| description | string | Mod description |
| creator_name | string | Username of the mod creator |
| votes | int | Number of upvotes |
| featured | bool | Whether the mod is featured |
| version | int | Mod version number |
| created_at | string | When the mod was first published |
| updated_at | string | When the mod was last updated |

---

## Market Endpoints

### Get Market Listings by Skin Index

Retrieve market information and active listings for a specific skin.

```
GET /api/market/skin/:skinindex
```

**Parameters:**
| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| skinindex | int | path | yes | The skin index (item type ID) |
| page | int | query | no | Page number for listings (default: 1) |

**Response:**
```json
{
  "skin_index": 123,
  "total_listings": 45,
  "lowest_price": 500,
  "average_price": 750.5,
  "total_circulating": 3500,
  "listings": [
    {
      "price": 500,
      "seller_name": "Player1",
      "listed_at": "2025-01-15T10:30:00Z"
    }
  ],
  "owners": [
    {
      "player_name": "Collector1",
      "count": 50
    },
    {
      "player_name": "Collector2",
      "count": 25
    }
  ],
  "price_history": [
    {
      "date": "2025-12-01",
      "average_price": 32500.5,
      "sales": 12
    },
    {
      "date": "2025-12-02",
      "average_price": 33000.0,
      "sales": 8
    }
  ]
}
```

**Response Fields:**
| Field | Type | Description |
|-------|------|-------------|
| skin_index | int | The skin index queried |
| total_listings | int | Total number of active listings |
| lowest_price | int | Lowest listed price (0 if no listings) |
| average_price | float | Average sale price from last 7 days |
| total_circulating | int | Total quantity of this skin in circulation |
| listings | object[] | Paginated list of active listings (sorted by price ascending) |
| listings[].price | int | Listing price in KR |
| listings[].seller_name | string | Seller's username |
| listings[].listed_at | string | When the listing was created |
| owners | object[] | First 100 owners sorted by quantity owned (descending) |
| owners[].player_name | string | Owner's username |
| owners[].count | int | Number of this skin the player owns |
| price_history | object[] | Daily average sale prices for the last 30 days |
| price_history[].date | string | Date (YYYY-MM-DD format) |
| price_history[].average_price | float | Average sale price on that day |
| price_history[].sales | int | Number of sales on that day |

**Notes:**
- Listings are sorted by price (lowest first)
- Page size is fixed at 10 records for listings
- `average_price` is calculated from sales in the last 7 days
- Owners list shows up to 100 players, sorted by quantity owned (highest first)
- Banned users are excluded from the owners list
- `price_history` contains daily averages for days with sales in the last 30 days (useful for charting)

---

## Error Responses

All endpoints return standard error responses:

| Status Code | Description |
|-------------|-------------|
| 400 | Bad request (missing or invalid parameters) |
| 403 | Forbidden (invalid API key) |
| 404 | Resource not found |
| 429 | Rate limit exceeded |
| 500 | Internal server error |

**Error Response Format:**
```json
{
  "error": "Error message description"
}
```
