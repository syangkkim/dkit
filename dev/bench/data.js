window.BENCHMARK_DATA = {
  "lastUpdate": 1774390648050,
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
      },
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
          "id": "d828381758f2d309fdb019e21df6e1d56a3369bc",
          "message": "Merge pull request #169 from syangkkim/claude/next-issue-XuqOJ",
          "timestamp": "2026-03-25T07:08:47+09:00",
          "tree_id": "b299b8d9f068ecc1f50cea651b1719a4d8ff0df4",
          "url": "https://github.com/syangkkim/dkit/commit/d828381758f2d309fdb019e21df6e1d56a3369bc"
        },
        "date": 1774390647666,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 1656132,
            "range": "± 73461",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 16551528,
            "range": "± 123775",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 1673492,
            "range": "± 65233",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 22349534,
            "range": "± 301279",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 1518442,
            "range": "± 15596",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 15248580,
            "range": "± 77103",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 1677861,
            "range": "± 21798",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 16968212,
            "range": "± 114219",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 636185,
            "range": "± 1708",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 603725,
            "range": "± 16058",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 523683,
            "range": "± 5244",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 995647,
            "range": "± 14221",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 14290707,
            "range": "± 590607",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 656515,
            "range": "± 8869",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 11528453,
            "range": "± 839142",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 993606,
            "range": "± 4024",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 9916449,
            "range": "± 72702",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 609735,
            "range": "± 19251",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 6082359,
            "range": "± 189473",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 475278,
            "range": "± 2702",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 2367367,
            "range": "± 14539",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 262843,
            "range": "± 8781",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 1324566,
            "range": "± 8507",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 493232,
            "range": "± 2430",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 11119151,
            "range": "± 235372",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1264602,
            "range": "± 9779",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 20082089,
            "range": "± 915611",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 902092,
            "range": "± 3927",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 19201653,
            "range": "± 282766",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 522416,
            "range": "± 5375",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10571559,
            "range": "± 366372",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 713821,
            "range": "± 2041",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 7315482,
            "range": "± 101689",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1221281,
            "range": "± 17973",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 18179148,
            "range": "± 345445",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 332,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 357,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 776,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 604,
            "range": "± 3",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}