In this document, there is the JSON representation of the flow already made using n8n:

```
{
  "nodes": [
    {
      "parameters": {
        "rule": {
          "interval": [
            {
              "triggerAtHour": 9
            }
          ]
        }
      },
      "type": "n8n-nodes-base.scheduleTrigger",
      "typeVersion": 1.2,
      "position": [
        -3200,
        208
      ],
      "id": "ff1f456a-5c9d-42f5-98fd-2e9621489f18",
      "name": "Schedule Trigger"
    },
    {
      "parameters": {
        "url": "https://octopusenergy.it/le-nostre-tariffe",
        "options": {}
      },
      "type": "n8n-nodes-base.httpRequest",
      "typeVersion": 4.2,
      "position": [
        -2976,
        112
      ],
      "id": "a1798284-bd64-47ce-bc88-65c4d2e2cca6",
      "name": "HTTP Request"
    },
    {
      "parameters": {
        "method": "POST",
        "url": "https://octopusenergy.it/api/auth/login",
        "authentication": "genericCredentialType",
        "genericAuthType": "httpCustomAuth",
        "options": {
          "response": {
            "response": {
              "fullResponse": true
            }
          }
        }
      },
      "type": "n8n-nodes-base.httpRequest",
      "typeVersion": 4.2,
      "position": [
        -2528,
        304
      ],
      "id": "20ff9956-4a2a-4898-b543-8e0608134ce3",
      "name": "Octopus login",
      "credentials": {
        "httpCustomAuth": {
          "id": "AQa9Rvd9qIkefrPY",
          "name": "Octopus Energy - Matteo"
        }
      }
    },
    {
      "parameters": {
        "mode": "runOnceForEachItem",
        "jsCode": "const accessTokenCookie = $input.item.json.headers['set-cookie']\n    .filter(item => item.includes('accessToken'))\n    .sort((a, b) => b.length - a.length)[0]\n\nconst accessToken = accessTokenCookie.substring(\n  accessTokenCookie.indexOf('=') + 1,\n  accessTokenCookie.indexOf(';')\n)\n\nreturn {\n  accessToken\n};"
      },
      "type": "n8n-nodes-base.code",
      "typeVersion": 2,
      "position": [
        -2304,
        304
      ],
      "id": "4853e464-20d0-4689-a610-8cd4700a4297",
      "name": "Extract accessToken"
    },
    {
      "parameters": {
        "operation": "extractHtmlContent",
        "extractionValues": {
          "values": [
            {
              "key": "products",
              "cssSelector": "#__NEXT_DATA__",
              "returnValue": "html"
            }
          ]
        },
        "options": {
          "trimValues": true,
          "cleanUpText": true
        }
      },
      "type": "n8n-nodes-base.html",
      "typeVersion": 1.2,
      "position": [
        -2752,
        112
      ],
      "id": "183520c3-3daf-4599-85fe-963b20986964",
      "name": "Extract tariffs from HTML"
    },
    {
      "parameters": {
        "mode": "runOnceForEachItem",
        "jsCode": "return JSON.parse($input.item.json.products).props.pageProps;"
      },
      "type": "n8n-nodes-base.code",
      "typeVersion": 2,
      "position": [
        -2528,
        112
      ],
      "id": "5bf5c62f-9a80-4fa5-8ac3-bb960d2c4eb5",
      "name": "Convert tariffs HTML content to JSON"
    },
    {
      "parameters": {
        "conditions": {
          "options": {
            "caseSensitive": true,
            "leftValue": "",
            "typeValidation": "strict",
            "version": 2
          },
          "conditions": [
            {
              "id": "2e969a3f-01f2-4994-84e2-18d42ebf670b",
              "leftValue": "={{ $json.params.productType }}",
              "rightValue": "FIXED_SINGLE_RATE",
              "operator": {
                "type": "string",
                "operation": "equals"
              }
            }
          ],
          "combinator": "and"
        },
        "options": {}
      },
      "type": "n8n-nodes-base.filter",
      "typeVersion": 2.2,
      "position": [
        -2080,
        112
      ],
      "id": "a7e34fce-3ab8-4a37-8a49-1709a4c14758",
      "name": "Retrieve 12M offers"
    },
    {
      "parameters": {
        "endpoint": "https://api.oeit-kraken.energy/v1/graphql/",
        "query": "query Viewer {\n  viewer {\n    email\n    fullName\n    accounts {\n      ... on AccountType {\n        number\n        properties {\n          electricitySupplyPoints {\n            status\n            product {\n              displayName\n              params {\n                consumptionCharge\n                annualStandingCharge\n                productType\n              }\n            }\n          }\n          gasSupplyPoints {\n            status\n            product {\n              params {\n                consumptionCharge\n                annualStandingCharge\n                productType\n              }\n            }\n          }\n        }\n      }\n    }\n  }\n}",
        "headerParametersUi": {
          "parameter": [
            {
              "name": "Authorization",
              "value": "={{ $json.accessToken }}"
            }
          ]
        }
      },
      "type": "n8n-nodes-base.graphql",
      "typeVersion": 1.1,
      "position": [
        -2080,
        304
      ],
      "id": "fb1654f3-4425-4224-abc1-c4b483abfe06",
      "name": "Query user's data"
    },
    {
      "parameters": {
        "jsCode": "return $input.all()[0].json.products;"
      },
      "type": "n8n-nodes-base.code",
      "typeVersion": 2,
      "position": [
        -2304,
        112
      ],
      "id": "2357ddb6-5e4f-47aa-9b90-4d9389def8bd",
      "name": "Take only products"
    },
    {
      "parameters": {
        "jsCode": "const adaptSupplyPoint = supplyPoint => ({\n  annualPrice: Number(supplyPoint.product.params.annualStandingCharge),\nconsumptionPrice: Number(supplyPoint.product.params.consumptionCharge.replace(',', '.'))\n})\n\nreturn $input.all().flatMap(\n  item => item.json.data.viewer.accounts.flatMap(\n    account => account.properties.flatMap(\n      accountProperty => accountProperty.electricitySupplyPoints.map(\n        electricitySupplyPoint => ({\n          type: 'luce',\n          ...adaptSupplyPoint(electricitySupplyPoint)\n        })\n      ).concat(\n        accountProperty.gasSupplyPoints.map(gasSupplyPoint => ({\n          type: 'gas',\n          ...adaptSupplyPoint(gasSupplyPoint)\n        }))\n      ).map(item => ({...item, accountNumber: account.number}))\n    )\n  )\n)"
      },
      "type": "n8n-nodes-base.code",
      "typeVersion": 2,
      "position": [
        -1856,
        304
      ],
      "id": "0cd844bb-4fdf-4797-becc-a4212a59fe2d",
      "name": "Flat accounts data"
    },
    {
      "parameters": {
        "assignments": {
          "assignments": [
            {
              "id": "bd8e845b-5c21-4220-a579-513cb39da6a7",
              "name": "type",
              "value": "={{ $json.__typename === 'ElectricityProductType' ? 'luce' : 'gas' }}",
              "type": "string"
            },
            {
              "id": "97045077-141b-4689-8826-68e3cdc6ae75",
              "name": "newAnnualPrice",
              "value": "={{ $json.params.annualStandingCharge }}",
              "type": "number"
            },
            {
              "id": "f6c282a7-148a-4664-bf7b-8ffef70b7065",
              "name": "newConsumptionPrice",
              "value": "={{ $json.params.consumptionCharge.replace(',', '.') }}",
              "type": "number"
            }
          ]
        },
        "options": {}
      },
      "type": "n8n-nodes-base.set",
      "typeVersion": 3.4,
      "position": [
        -1856,
        112
      ],
      "id": "a8792ee0-7561-44b9-97d9-458dc3494a3c",
      "name": "Transform tariffs"
    },
    {
      "parameters": {
        "mode": "combine",
        "fieldsToMatchString": "type",
        "options": {}
      },
      "type": "n8n-nodes-base.merge",
      "typeVersion": 3.2,
      "position": [
        -1632,
        208
      ],
      "id": "2cf03edd-05d8-4dd6-a041-73c7d7ccbdbc",
      "name": "Merge new tariffs with actual one"
    },
    {
      "parameters": {
        "conditions": {
          "options": {
            "caseSensitive": true,
            "leftValue": "",
            "typeValidation": "strict",
            "version": 2
          },
          "conditions": [
            {
              "id": "a9ecf3f6-0dd3-4d3d-acae-9349b20244e3",
              "leftValue": "={{ $json.newAnnualPrice }}",
              "rightValue": "={{ $json.annualPrice }}",
              "operator": {
                "type": "number",
                "operation": "lte"
              }
            },
            {
              "id": "dad2178c-9638-4753-b693-20efe79b1696",
              "leftValue": "={{ $json.newConsumptionPrice }}",
              "rightValue": "={{ $json.consumptionPrice }}",
              "operator": {
                "type": "number",
                "operation": "lte"
              }
            },
            {
              "id": "502d5bc2-b2cb-452c-ab13-d39ffb4d6213",
              "leftValue": "={{ $json.newAnnualPrice }}",
              "rightValue": "={{ $json.value?.annualPrice ?? $json.annualPrice }}",
              "operator": {
                "type": "number",
                "operation": "lte"
              }
            },
            {
              "id": "f03da11e-1ab3-4116-8273-1cf71d81b473",
              "leftValue": "={{ $json.newConsumptionPrice }}",
              "rightValue": "={{ $json.value?.consumptionPrice ?? $json.consumptionPrice }}",
              "operator": {
                "type": "number",
                "operation": "lt"
              }
            }
          ],
          "combinator": "and"
        },
        "options": {}
      },
      "type": "n8n-nodes-base.if",
      "typeVersion": 2.2,
      "position": [
        -960,
        208
      ],
      "id": "5ba24ce0-5e2e-4385-bb74-608cf23ac28c",
      "name": "Check if needs to request update"
    },
    {
      "parameters": {
        "keyName": "={{ $json.accountNumber }}-{{ $json.type }}",
        "valueDataType": "json",
        "valueJson": "={\n  \"annualPrice\": {{ $json.newAnnualPrice }},\n  \"consumptionPrice\": {{ $json.newConsumptionPrice }},\n  \"type\": \"{{ $json.type }}\"\n}"
      },
      "type": "n8n-nodes-datastore.datastore",
      "typeVersion": 1,
      "position": [
        -736,
        112
      ],
      "id": "9950c634-22c6-4248-86bc-57d2a8b95ebd",
      "name": "Cache new data"
    },
    {
      "parameters": {
        "fromEmail": "={{ $('Query user\\'s data').item.json.data.viewer.email }}",
        "toEmail": "ciao@octopusenergy.it",
        "subject": "=Richiesta adeguamento tariffa {{ $('Merge new tariffs with actual one').item.json.type }} - account {{ $('Merge new tariffs with actual one').item.json.accountNumber }}",
        "emailFormat": "text",
        "text": "=Buongiorno,\ncon la presente richiedo l'adeguamento della mia tariffa {{ $('Merge new tariffs with actual one').item.json.type }} con quella attualmente in commercio, per l'account {{ $('Merge new tariffs with actual one').item.json.accountNumber }}.\nIn dettaglio, vorrei passare dalla mia tariffa da {{ $('Merge new tariffs with actual one').item.json.consumptionPrice }} a quella da {{ $('Merge new tariffs with actual one').item.json.newConsumptionPrice }}.\n\nCordiali saluti,\n{{ $('Query user\\'s data').item.json.data.viewer.fullName }}",
        "options": {
          "appendAttribution": false
        }
      },
      "type": "n8n-nodes-base.emailSend",
      "typeVersion": 2.1,
      "position": [
        -736,
        304
      ],
      "id": "4e307dc5-6682-4341-9752-e2593784b55b",
      "name": "Send email",
      "webhookId": "95b19ba7-a38b-400c-9c72-218eb52a1676",
      "credentials": {
        "smtp": {
          "id": "p6GZHm8Mx1ljJxBO",
          "name": "GMail account"
        }
      }
    },
    {
      "parameters": {
        "operation": "get",
        "keyName": "={{ $json.accountNumber }}-{{ $json.type }}"
      },
      "type": "n8n-nodes-datastore.datastore",
      "typeVersion": 1,
      "position": [
        -1408,
        288
      ],
      "id": "90daeba9-7535-4949-bd78-e6ab22c92160",
      "name": "Get cached data"
    },
    {
      "parameters": {
        "content": "External dependency:\nn8n-nodes-datastore"
      },
      "type": "n8n-nodes-base.stickyNote",
      "position": [
        -1696,
        -32
      ],
      "typeVersion": 1,
      "id": "ddd19340-191b-40ed-8951-4f1c3fe73d2d",
      "name": "Sticky Note"
    },
    {
      "parameters": {
        "mode": "combine",
        "advanced": true,
        "mergeByFields": {
          "values": [
            {
              "field1": "type",
              "field2": "value.type"
            }
          ]
        },
        "joinMode": "enrichInput1",
        "options": {}
      },
      "type": "n8n-nodes-base.merge",
      "typeVersion": 3.2,
      "position": [
        -1184,
        208
      ],
      "id": "f33b1205-adf0-415c-ac17-e74815fc51c4",
      "name": "Merge tariffs with cached data"
    }
  ],
  "connections": {
    "Schedule Trigger": {
      "main": [
        [
          {
            "node": "HTTP Request",
            "type": "main",
            "index": 0
          },
          {
            "node": "Octopus login",
            "type": "main",
            "index": 0
          }
        ]
      ]
    },
    "HTTP Request": {
      "main": [
        [
          {
            "node": "Extract tariffs from HTML",
            "type": "main",
            "index": 0
          }
        ]
      ]
    },
    "Octopus login": {
      "main": [
        [
          {
            "node": "Extract accessToken",
            "type": "main",
            "index": 0
          }
        ]
      ]
    },
    "Extract accessToken": {
      "main": [
        [
          {
            "node": "Query user's data",
            "type": "main",
            "index": 0
          }
        ]
      ]
    },
    "Extract tariffs from HTML": {
      "main": [
        [
          {
            "node": "Convert tariffs HTML content to JSON",
            "type": "main",
            "index": 0
          }
        ]
      ]
    },
    "Convert tariffs HTML content to JSON": {
      "main": [
        [
          {
            "node": "Take only products",
            "type": "main",
            "index": 0
          }
        ]
      ]
    },
    "Retrieve 12M offers": {
      "main": [
        [
          {
            "node": "Transform tariffs",
            "type": "main",
            "index": 0
          }
        ]
      ]
    },
    "Query user's data": {
      "main": [
        [
          {
            "node": "Flat accounts data",
            "type": "main",
            "index": 0
          }
        ]
      ]
    },
    "Take only products": {
      "main": [
        [
          {
            "node": "Retrieve 12M offers",
            "type": "main",
            "index": 0
          }
        ]
      ]
    },
    "Flat accounts data": {
      "main": [
        [
          {
            "node": "Merge new tariffs with actual one",
            "type": "main",
            "index": 1
          }
        ]
      ]
    },
    "Transform tariffs": {
      "main": [
        [
          {
            "node": "Merge new tariffs with actual one",
            "type": "main",
            "index": 0
          }
        ]
      ]
    },
    "Merge new tariffs with actual one": {
      "main": [
        [
          {
            "node": "Merge tariffs with cached data",
            "type": "main",
            "index": 0
          },
          {
            "node": "Get cached data",
            "type": "main",
            "index": 0
          }
        ]
      ]
    },
    "Check if needs to request update": {
      "main": [
        [
          {
            "node": "Cache new data",
            "type": "main",
            "index": 0
          },
          {
            "node": "Send email",
            "type": "main",
            "index": 0
          }
        ]
      ]
    },
    "Get cached data": {
      "main": [
        [
          {
            "node": "Merge tariffs with cached data",
            "type": "main",
            "index": 1
          }
        ]
      ]
    },
    "Merge tariffs with cached data": {
      "main": [
        [
          {
            "node": "Check if needs to request update",
            "type": "main",
            "index": 0
          }
        ]
      ]
    }
  },
  "pinData": {},
  "meta": {
    "templateCredsSetupCompleted": true,
    "instanceId": "d0f8ec875160cccbcd01fe0c035646b01d96e5f5cbf1f42b1e49b591877afe1a"
  }
}
```