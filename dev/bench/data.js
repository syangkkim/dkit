window.BENCHMARK_DATA = {
  "lastUpdate": 1774369343614,
  "repoUrl": "https://github.com/syangkkim/dkit",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "syang.kkim@gmail.com",
            "name": "syangkkim",
            "username": "syangkkim"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3b24db2e8803808a268a08798aca4006916e07c8",
          "message": "Merge pull request #168 from syangkkim/claude/milestone-v1-0-0-start-cI9Fs",
          "timestamp": "2026-03-25T01:11:06+09:00",
          "tree_id": "931a90d4fe1c0b81f9a5aa1e4b1f9abb37b6feea",
          "url": "https://github.com/syangkkim/dkit/commit/3b24db2e8803808a268a08798aca4006916e07c8"
        },
        "date": 1774369342553,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 1650543,
            "range": "± 20926",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 19523500,
            "range": "± 533734",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 1677833,
            "range": "± 85412",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 24093148,
            "range": "± 266222",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 1520795,
            "range": "± 20549",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 17356775,
            "range": "± 417665",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 1669278,
            "range": "± 26925",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 16924349,
            "range": "± 703319",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 637701,
            "range": "± 2167",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 625610,
            "range": "± 7946",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 505591,
            "range": "± 19464",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 994745,
            "range": "± 13811",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17055424,
            "range": "± 418363",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 617718,
            "range": "± 6688",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 12801521,
            "range": "± 282690",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1028634,
            "range": "± 12638",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 10344389,
            "range": "± 140546",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 631112,
            "range": "± 27075",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 6332570,
            "range": "± 124495",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 483305,
            "range": "± 1500",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 2408808,
            "range": "± 24859",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 262123,
            "range": "± 3587",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 1326313,
            "range": "± 21080",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 494530,
            "range": "± 1783",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 13172756,
            "range": "± 386082",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1264176,
            "range": "± 18351",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 23983923,
            "range": "± 1325022",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 910475,
            "range": "± 5400",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 26234854,
            "range": "± 1521687",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 522075,
            "range": "± 1655",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 11791552,
            "range": "± 749560",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 717703,
            "range": "± 7910",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 8323695,
            "range": "± 399773",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1216933,
            "range": "± 20366",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 24998033,
            "range": "± 1008443",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 321,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 348,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 758,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 603,
            "range": "± 4",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}