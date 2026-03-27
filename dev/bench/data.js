window.BENCHMARK_DATA = {
  "lastUpdate": 1774613195508,
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
          "id": "27599defb539fe5d01caa1cb1bbd2e7eaa2ed534",
          "message": "Merge pull request #170 from syangkkim/claude/next-issue-N5fba",
          "timestamp": "2026-03-25T07:28:14+09:00",
          "tree_id": "620f2274ff4fbc49e76afb76875af474ccc8944a",
          "url": "https://github.com/syangkkim/dkit/commit/27599defb539fe5d01caa1cb1bbd2e7eaa2ed534"
        },
        "date": 1774391892834,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2116792,
            "range": "± 9128",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 21444547,
            "range": "± 64782",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2267571,
            "range": "± 75087",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 27824733,
            "range": "± 615523",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2096470,
            "range": "± 7861",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21049263,
            "range": "± 85350",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2162222,
            "range": "± 7661",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 21662989,
            "range": "± 56463",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 904242,
            "range": "± 7984",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 805996,
            "range": "± 2486",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 783134,
            "range": "± 2702",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1304039,
            "range": "± 17419",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17435299,
            "range": "± 187847",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 962654,
            "range": "± 2658",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 14450224,
            "range": "± 106844",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1382678,
            "range": "± 15678",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13701709,
            "range": "± 77055",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 797847,
            "range": "± 5855",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8039143,
            "range": "± 33409",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 862911,
            "range": "± 1875",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4264969,
            "range": "± 10559",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 497284,
            "range": "± 1287",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2494277,
            "range": "± 6336",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 597526,
            "range": "± 2183",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 11914920,
            "range": "± 125747",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1802397,
            "range": "± 17778",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 24011028,
            "range": "± 249576",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1179011,
            "range": "± 19294",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 25174197,
            "range": "± 362679",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 597858,
            "range": "± 2037",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10924580,
            "range": "± 138670",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 876620,
            "range": "± 2489",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 8898878,
            "range": "± 68791",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1833924,
            "range": "± 12339",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 22848409,
            "range": "± 315386",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 462,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 621,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1212,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 994,
            "range": "± 3",
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
          "id": "8224ed3ef33ce0387bbbab4d9729f8dc5621739d",
          "message": "Merge pull request #171 from syangkkim/claude/next-issue-ggwEU",
          "timestamp": "2026-03-25T08:09:47+09:00",
          "tree_id": "5c33cb8ae3e3fa045efa1a3c84f650e45be41b07",
          "url": "https://github.com/syangkkim/dkit/commit/8224ed3ef33ce0387bbbab4d9729f8dc5621739d"
        },
        "date": 1774394322754,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2135084,
            "range": "± 7301",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 21862864,
            "range": "± 417926",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2281359,
            "range": "± 105652",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 32312571,
            "range": "± 788254",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2095407,
            "range": "± 9553",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21244177,
            "range": "± 459601",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2170785,
            "range": "± 23500",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 21915342,
            "range": "± 152521",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 896772,
            "range": "± 6937",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 788779,
            "range": "± 2789",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 793988,
            "range": "± 8174",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1294968,
            "range": "± 23386",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17616261,
            "range": "± 332431",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 920631,
            "range": "± 7476",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 14451899,
            "range": "± 333968",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1359215,
            "range": "± 16428",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13407783,
            "range": "± 216138",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 795971,
            "range": "± 2002",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 7971975,
            "range": "± 64708",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 864439,
            "range": "± 3031",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4314230,
            "range": "± 38983",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 489671,
            "range": "± 11061",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2458746,
            "range": "± 6883",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 600835,
            "range": "± 7242",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 12569218,
            "range": "± 397771",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1781434,
            "range": "± 30882",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 25832935,
            "range": "± 1214808",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1167644,
            "range": "± 22863",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 27819849,
            "range": "± 1431896",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 596019,
            "range": "± 2101",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 11591954,
            "range": "± 520125",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 880365,
            "range": "± 4505",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 8970557,
            "range": "± 107645",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1673696,
            "range": "± 9900",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 24485076,
            "range": "± 1784214",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 484,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 651,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1255,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 998,
            "range": "± 6",
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
          "id": "3391d1be775081c7a726531c390b493481356022",
          "message": "Merge pull request #172 from syangkkim/claude/next-issue-278UX",
          "timestamp": "2026-03-25T08:20:52+09:00",
          "tree_id": "a90a0d178c9fc31259087262bde8485e2ab62929",
          "url": "https://github.com/syangkkim/dkit/commit/3391d1be775081c7a726531c390b493481356022"
        },
        "date": 1774394977450,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2131319,
            "range": "± 8808",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22259770,
            "range": "± 743975",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2412564,
            "range": "± 69626",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 30501646,
            "range": "± 1002951",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2208693,
            "range": "± 16155",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 22352503,
            "range": "± 287332",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2147744,
            "range": "± 15195",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 21636341,
            "range": "± 553484",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 1012910,
            "range": "± 39884",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 786199,
            "range": "± 4954",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 910356,
            "range": "± 2758",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1310762,
            "range": "± 19624",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17900459,
            "range": "± 458313",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 931222,
            "range": "± 4434",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 14794889,
            "range": "± 414734",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1336109,
            "range": "± 10105",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13677773,
            "range": "± 555146",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 794033,
            "range": "± 4276",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8037699,
            "range": "± 82807",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 875910,
            "range": "± 19559",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4288053,
            "range": "± 77190",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 490901,
            "range": "± 5769",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2464480,
            "range": "± 6394",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 601390,
            "range": "± 3131",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 13465991,
            "range": "± 348741",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1774760,
            "range": "± 22719",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 27021483,
            "range": "± 1518663",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1173997,
            "range": "± 17385",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 29902776,
            "range": "± 2007113",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 601584,
            "range": "± 2222",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 11713243,
            "range": "± 317362",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 880222,
            "range": "± 15429",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9218277,
            "range": "± 151288",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1675931,
            "range": "± 15409",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 24961772,
            "range": "± 670379",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 466,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 629,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1223,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1007,
            "range": "± 5",
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
          "id": "db6d4203abf963edcbd2e645d7f637d24d974ada",
          "message": "Merge pull request #173 from syangkkim/claude/next-issue-7V7nL\n\nfeat: add feature flags for optional format dependencies",
          "timestamp": "2026-03-25T08:47:44+09:00",
          "tree_id": "33db0ad2c1cd8d75d74189fc1f06312d3401f09f",
          "url": "https://github.com/syangkkim/dkit/commit/db6d4203abf963edcbd2e645d7f637d24d974ada"
        },
        "date": 1774396593461,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2211182,
            "range": "± 5190",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22338216,
            "range": "± 226353",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2311237,
            "range": "± 51330",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 32272899,
            "range": "± 567207",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2193826,
            "range": "± 9469",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 22189281,
            "range": "± 111447",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2215725,
            "range": "± 7397",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22149994,
            "range": "± 77303",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 956724,
            "range": "± 6296",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 811137,
            "range": "± 2008",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 864168,
            "range": "± 6967",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1405813,
            "range": "± 8492",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 19108332,
            "range": "± 58987",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1016683,
            "range": "± 14479",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 15944578,
            "range": "± 95981",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1290140,
            "range": "± 15202",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 12765947,
            "range": "± 56157",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 810353,
            "range": "± 2933",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8187603,
            "range": "± 163217",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 814054,
            "range": "± 2171",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4066019,
            "range": "± 14353",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 454206,
            "range": "± 1405",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2295113,
            "range": "± 9158",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 555527,
            "range": "± 2296",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 12693222,
            "range": "± 86002",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1764535,
            "range": "± 26891",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 25629550,
            "range": "± 137136",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1158843,
            "range": "± 17587",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 27074224,
            "range": "± 185820",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 585078,
            "range": "± 1391",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 11771013,
            "range": "± 117462",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 841878,
            "range": "± 3676",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 8612450,
            "range": "± 75976",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1676621,
            "range": "± 24318",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 24106201,
            "range": "± 102812",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 497,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 687,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1257,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1120,
            "range": "± 5",
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
          "id": "f022f2b62a917cc1f112c7c638570f7dca9e62cd",
          "message": "Merge pull request #174 from syangkkim/claude/next-issue-K0I4N\n\nAdd v1.0.0 stabilization QA test suite",
          "timestamp": "2026-03-25T09:03:53+09:00",
          "tree_id": "5a99ba3db97a86e22f55a82f8b567fb577c153d1",
          "url": "https://github.com/syangkkim/dkit/commit/f022f2b62a917cc1f112c7c638570f7dca9e62cd"
        },
        "date": 1774397559045,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2173552,
            "range": "± 64978",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22044668,
            "range": "± 489749",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2318746,
            "range": "± 61833",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 28784398,
            "range": "± 537316",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2139213,
            "range": "± 7842",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21639289,
            "range": "± 504778",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2205693,
            "range": "± 13566",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22108451,
            "range": "± 117762",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 901159,
            "range": "± 3407",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 807948,
            "range": "± 3890",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 803128,
            "range": "± 23802",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1355412,
            "range": "± 14770",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 18409959,
            "range": "± 684850",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 955226,
            "range": "± 3770",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 14565613,
            "range": "± 399390",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1335461,
            "range": "± 21566",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13369935,
            "range": "± 290790",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 812523,
            "range": "± 2432",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 7978724,
            "range": "± 69559",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 862059,
            "range": "± 5522",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4301765,
            "range": "± 18567",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 494286,
            "range": "± 2858",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2483851,
            "range": "± 12339",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 606647,
            "range": "± 10149",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 12507742,
            "range": "± 606280",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1796401,
            "range": "± 17998",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 24487971,
            "range": "± 1225346",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1182668,
            "range": "± 18356",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 26352945,
            "range": "± 1484285",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 608152,
            "range": "± 1846",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 11507085,
            "range": "± 609791",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 893064,
            "range": "± 3217",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9257731,
            "range": "± 245021",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1689545,
            "range": "± 38333",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 24125063,
            "range": "± 1572651",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 509,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 692,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1314,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1095,
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
          "id": "7096b901b97ea3e0533099a6392745e1fbc22961",
          "message": "Merge pull request #178 from syangkkim/claude/next-issue-bx7Nv\n\nAdd cross-platform CI test matrix with feature flag combinations",
          "timestamp": "2026-03-25T09:20:50+09:00",
          "tree_id": "05c6f13ebd07955c1d31ec2309fe4569cda07814",
          "url": "https://github.com/syangkkim/dkit/commit/7096b901b97ea3e0533099a6392745e1fbc22961"
        },
        "date": 1774398584734,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2181452,
            "range": "± 104644",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22082916,
            "range": "± 105546",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2296820,
            "range": "± 50558",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 27641606,
            "range": "± 112834",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2177276,
            "range": "± 15590",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21749632,
            "range": "± 72211",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2231614,
            "range": "± 35396",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22271248,
            "range": "± 103674",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 901449,
            "range": "± 4208",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 817423,
            "range": "± 8218",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 793037,
            "range": "± 6727",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1351394,
            "range": "± 26236",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17718570,
            "range": "± 34488",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 954707,
            "range": "± 3541",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 14119021,
            "range": "± 54868",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1346151,
            "range": "± 25618",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13321756,
            "range": "± 35055",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 801236,
            "range": "± 2606",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 7998390,
            "range": "± 24677",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 855541,
            "range": "± 12462",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4459292,
            "range": "± 32507",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 498267,
            "range": "± 1861",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2490727,
            "range": "± 10713",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 603468,
            "range": "± 2044",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 11763234,
            "range": "± 30081",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1789035,
            "range": "± 19646",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 23499623,
            "range": "± 64011",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1183198,
            "range": "± 17677",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 24769970,
            "range": "± 111387",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 601530,
            "range": "± 1585",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10767874,
            "range": "± 47359",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 877936,
            "range": "± 2340",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 8921088,
            "range": "± 60103",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1676243,
            "range": "± 17075",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 22307954,
            "range": "± 53682",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 509,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 707,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1295,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1077,
            "range": "± 6",
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
          "id": "d42c604a1133b6ad673984d0543fa6ad2996eafa",
          "message": "Merge pull request #179 from syangkkim/claude/next-issue-2Wa7P\n\nAdd Miri memory safety CI workflow",
          "timestamp": "2026-03-25T09:27:20+09:00",
          "tree_id": "e3cb63e2bc8e993cd65ac8227e6b4e871fee3602",
          "url": "https://github.com/syangkkim/dkit/commit/d42c604a1133b6ad673984d0543fa6ad2996eafa"
        },
        "date": 1774398965789,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 1775690,
            "range": "± 45874",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 20133967,
            "range": "± 121905",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2092862,
            "range": "± 43781",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 26388881,
            "range": "± 507341",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 1798074,
            "range": "± 16076",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 20069458,
            "range": "± 1047429",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 1969353,
            "range": "± 45508",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 20396382,
            "range": "± 116401",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 839950,
            "range": "± 1636",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 749318,
            "range": "± 3003",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 658513,
            "range": "± 1429",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1106142,
            "range": "± 13142",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 14872207,
            "range": "± 362900",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 872641,
            "range": "± 2304",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 11951133,
            "range": "± 300557",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1217990,
            "range": "± 12439",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 12624651,
            "range": "± 131253",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 732236,
            "range": "± 1327",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 7505725,
            "range": "± 42739",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 751099,
            "range": "± 3963",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 3773042,
            "range": "± 27142",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 447854,
            "range": "± 733",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2251101,
            "range": "± 3118",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 476038,
            "range": "± 1161",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 10182860,
            "range": "± 190406",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1632956,
            "range": "± 11529",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 23269519,
            "range": "± 600467",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1124245,
            "range": "± 9475",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 24711486,
            "range": "± 648011",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 607271,
            "range": "± 2031",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 9662186,
            "range": "± 324010",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 832319,
            "range": "± 1810",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9376641,
            "range": "± 124359",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1551375,
            "range": "± 31520",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 20997098,
            "range": "± 375209",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 472,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 584,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1136,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 911,
            "range": "± 1",
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
          "id": "500a3ac672ba18c812b8d89d7eb4fe31f482956f",
          "message": "Merge pull request #180 from syangkkim/claude/next-issue-oXEZI\n\nAdd cargo-fuzz fuzzing infrastructure for all parsers",
          "timestamp": "2026-03-25T10:02:03+09:00",
          "tree_id": "6edf386fad6d71f1a02ab4c40000a237314befb9",
          "url": "https://github.com/syangkkim/dkit/commit/500a3ac672ba18c812b8d89d7eb4fe31f482956f"
        },
        "date": 1774401065632,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2179820,
            "range": "± 98334",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22172316,
            "range": "± 572713",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2314482,
            "range": "± 80647",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 31129247,
            "range": "± 508679",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2157950,
            "range": "± 63980",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21609731,
            "range": "± 497379",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2228586,
            "range": "± 7028",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22295415,
            "range": "± 1199296",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 912276,
            "range": "± 21204",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 822700,
            "range": "± 2547",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 807082,
            "range": "± 9329",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1343873,
            "range": "± 29208",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17858763,
            "range": "± 111824",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 963148,
            "range": "± 7336",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 14374985,
            "range": "± 590568",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1358199,
            "range": "± 55775",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13257290,
            "range": "± 63577",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 806931,
            "range": "± 6566",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8057060,
            "range": "± 85042",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 888840,
            "range": "± 18965",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4290684,
            "range": "± 154137",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 493331,
            "range": "± 20789",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2483574,
            "range": "± 26208",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 598623,
            "range": "± 15479",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 11871247,
            "range": "± 354712",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1789445,
            "range": "± 45263",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 24166600,
            "range": "± 1299154",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1173673,
            "range": "± 6924",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 28308327,
            "range": "± 2593067",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 604805,
            "range": "± 5493",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 13641780,
            "range": "± 788161",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 877616,
            "range": "± 4571",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 11898316,
            "range": "± 1672574",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1705241,
            "range": "± 11384",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 31861333,
            "range": "± 1509936",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 509,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 697,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1306,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1091,
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
          "id": "308814397737cf7f28377e83a5f7ea527ef27aba",
          "message": "Merge pull request #181 from syangkkim/claude/next-issue-0poMz\n\nchore: bump version to 1.0.0 for stable release",
          "timestamp": "2026-03-25T10:27:03+09:00",
          "tree_id": "e84e554dcaa754ff167c6241cd7f1d040d0c27ae",
          "url": "https://github.com/syangkkim/dkit/commit/308814397737cf7f28377e83a5f7ea527ef27aba"
        },
        "date": 1774402554351,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2166142,
            "range": "± 9825",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 21941822,
            "range": "± 86072",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2290980,
            "range": "± 110144",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 30720991,
            "range": "± 300787",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2122507,
            "range": "± 11150",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21412915,
            "range": "± 503723",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2214340,
            "range": "± 10216",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22154940,
            "range": "± 90321",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 904820,
            "range": "± 4906",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 812753,
            "range": "± 1872",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 800743,
            "range": "± 2077",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1366418,
            "range": "± 23890",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17862929,
            "range": "± 54977",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 941064,
            "range": "± 2358",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 14173475,
            "range": "± 201279",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1401351,
            "range": "± 31012",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13827640,
            "range": "± 70664",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 802185,
            "range": "± 8596",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8026413,
            "range": "± 35089",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 850562,
            "range": "± 3580",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4244165,
            "range": "± 10456",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 493534,
            "range": "± 7263",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2493282,
            "range": "± 5482",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 596080,
            "range": "± 9263",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 11685149,
            "range": "± 320761",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1782647,
            "range": "± 8763",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 23493177,
            "range": "± 151350",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1175343,
            "range": "± 19227",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 25044236,
            "range": "± 150827",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 606798,
            "range": "± 4096",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10715285,
            "range": "± 285995",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 879602,
            "range": "± 10705",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9043925,
            "range": "± 75724",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1675001,
            "range": "± 17004",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 22380867,
            "range": "± 646829",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 507,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 697,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1325,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1114,
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
          "id": "dda69e40b0a4c53a8cc600873a498c27ad8b6b3d",
          "message": "Merge pull request #182 from syangkkim/claude/fix-cargo-publish-error-EwawY\n\nFix cargo publish error for dkit-core dependency",
          "timestamp": "2026-03-25T10:49:46+09:00",
          "tree_id": "216f1fdd38d45781e5198c815b5098170c293069",
          "url": "https://github.com/syangkkim/dkit/commit/dda69e40b0a4c53a8cc600873a498c27ad8b6b3d"
        },
        "date": 1774403920125,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2165888,
            "range": "± 5015",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 21797195,
            "range": "± 521481",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2320639,
            "range": "± 103843",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 31226846,
            "range": "± 991713",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2138195,
            "range": "± 5617",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21616679,
            "range": "± 136411",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2210228,
            "range": "± 7940",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22060252,
            "range": "± 97432",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 913835,
            "range": "± 2996",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 797517,
            "range": "± 2851",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 799323,
            "range": "± 2105",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1339616,
            "range": "± 17465",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17810061,
            "range": "± 578334",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 939233,
            "range": "± 7710",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 14091859,
            "range": "± 109250",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1365332,
            "range": "± 10409",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13486766,
            "range": "± 374748",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 814091,
            "range": "± 4515",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8145076,
            "range": "± 30106",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 850804,
            "range": "± 7297",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4219620,
            "range": "± 29944",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 492875,
            "range": "± 4424",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2470566,
            "range": "± 5951",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 596873,
            "range": "± 1805",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 11921008,
            "range": "± 149304",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1794275,
            "range": "± 15293",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 24326414,
            "range": "± 737226",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1170208,
            "range": "± 17600",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 25404221,
            "range": "± 738507",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 602449,
            "range": "± 2873",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10909806,
            "range": "± 128667",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 885406,
            "range": "± 3543",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 8843759,
            "range": "± 78762",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1691568,
            "range": "± 22315",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 22943487,
            "range": "± 376105",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 508,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 700,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1301,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1087,
            "range": "± 11",
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
          "id": "3b7adcb95c9c7fef3bc986dca9f512bde3e5fbbf",
          "message": "Merge pull request #183 from syangkkim/claude/fix-cargo-publish-error-EwawY\n\nSkip cargo publish if version already exists on crates.io",
          "timestamp": "2026-03-25T11:10:21+09:00",
          "tree_id": "84165dc8385f490aec8a7c69b5d76b445b98e726",
          "url": "https://github.com/syangkkim/dkit/commit/3b7adcb95c9c7fef3bc986dca9f512bde3e5fbbf"
        },
        "date": 1774405147885,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2162403,
            "range": "± 6147",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22404210,
            "range": "± 1121280",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2291001,
            "range": "± 103828",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 29335176,
            "range": "± 629513",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2135533,
            "range": "± 6093",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 22020494,
            "range": "± 412927",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2210903,
            "range": "± 9310",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22361424,
            "range": "± 159543",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 889663,
            "range": "± 25910",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 807855,
            "range": "± 5722",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 785839,
            "range": "± 13847",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1361626,
            "range": "± 18410",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 18259757,
            "range": "± 392131",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 934671,
            "range": "± 3887",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 15067197,
            "range": "± 298660",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1391050,
            "range": "± 21276",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13801633,
            "range": "± 110704",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 795034,
            "range": "± 3337",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 7975621,
            "range": "± 23174",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 856200,
            "range": "± 6311",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4272850,
            "range": "± 22703",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 492482,
            "range": "± 5482",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2474072,
            "range": "± 5476",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 593121,
            "range": "± 3105",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 12560030,
            "range": "± 338161",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1785895,
            "range": "± 19043",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 25708621,
            "range": "± 1300930",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1174990,
            "range": "± 26482",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 25152695,
            "range": "± 1091120",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 603692,
            "range": "± 1855",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 11260585,
            "range": "± 411515",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 878549,
            "range": "± 5011",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 8923466,
            "range": "± 163940",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1702335,
            "range": "± 14839",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 24897301,
            "range": "± 1080340",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 512,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 699,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1317,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1131,
            "range": "± 55",
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
          "id": "efd83833373e2933b706e6959e4b6b1c7e58bee5",
          "message": "Merge pull request #223 from syangkkim/claude/next-issue-MBYlC\n\nfeat: add --select flag for convert/view (column selection)",
          "timestamp": "2026-03-25T19:36:01+09:00",
          "tree_id": "8286d1285301f19e38102fac6a9264550e91d78f",
          "url": "https://github.com/syangkkim/dkit/commit/efd83833373e2933b706e6959e4b6b1c7e58bee5"
        },
        "date": 1774435496503,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2162695,
            "range": "± 72597",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22034123,
            "range": "± 675654",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2287424,
            "range": "± 21330",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 32990468,
            "range": "± 562354",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2158741,
            "range": "± 11857",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21745728,
            "range": "± 555418",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2225160,
            "range": "± 47918",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22312152,
            "range": "± 402728",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 901289,
            "range": "± 5019",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 796350,
            "range": "± 2951",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 804487,
            "range": "± 23677",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1355735,
            "range": "± 20894",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 18250918,
            "range": "± 620722",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 953663,
            "range": "± 10608",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 14623731,
            "range": "± 481514",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1371920,
            "range": "± 12571",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13860432,
            "range": "± 290679",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 797195,
            "range": "± 2159",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8067182,
            "range": "± 36170",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 871374,
            "range": "± 8886",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4362799,
            "range": "± 73765",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 491060,
            "range": "± 1248",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2463951,
            "range": "± 14722",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 594397,
            "range": "± 2634",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 12294716,
            "range": "± 415446",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1785303,
            "range": "± 110525",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 25825674,
            "range": "± 2209548",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1187304,
            "range": "± 22750",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 25934373,
            "range": "± 1799351",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 605243,
            "range": "± 2480",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 11609993,
            "range": "± 501792",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 884638,
            "range": "± 6058",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9410765,
            "range": "± 504491",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1720329,
            "range": "± 27435",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 24539946,
            "range": "± 1484214",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 515,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 696,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1321,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1087,
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
          "id": "9446062eef2257e79fca2a737c5f340d283b7f25",
          "message": "Merge pull request #229 from syangkkim/claude/add-dkit-readmes-WrgI5\n\nchore: add crate READMEs, improve metadata, bump to v1.0.1",
          "timestamp": "2026-03-26T18:47:01+09:00",
          "tree_id": "06cf42ec4adf867b1c58e0eee3fb9991ef37faf5",
          "url": "https://github.com/syangkkim/dkit/commit/9446062eef2257e79fca2a737c5f340d283b7f25"
        },
        "date": 1774518970057,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2173367,
            "range": "± 4304",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22036503,
            "range": "± 78580",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2327671,
            "range": "± 25216",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 31207971,
            "range": "± 500423",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2175356,
            "range": "± 7097",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 23186905,
            "range": "± 290847",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2217562,
            "range": "± 9435",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 23709245,
            "range": "± 356105",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 919906,
            "range": "± 5338",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 803044,
            "range": "± 8020",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 815037,
            "range": "± 21111",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1351757,
            "range": "± 25490",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17931045,
            "range": "± 258936",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1002962,
            "range": "± 18088",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 15211923,
            "range": "± 373015",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1354516,
            "range": "± 15409",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13449718,
            "range": "± 33406",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 795477,
            "range": "± 1575",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 7930643,
            "range": "± 34196",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 872986,
            "range": "± 2685",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4236451,
            "range": "± 10053",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 493470,
            "range": "± 1372",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2480179,
            "range": "± 8631",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 597366,
            "range": "± 13335",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 11812836,
            "range": "± 133009",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1771089,
            "range": "± 37561",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 23336339,
            "range": "± 313476",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1160510,
            "range": "± 10741",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 24855887,
            "range": "± 558945",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 598840,
            "range": "± 14264",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10767587,
            "range": "± 189987",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 870922,
            "range": "± 8461",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 8858156,
            "range": "± 137832",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1659374,
            "range": "± 18713",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 22163853,
            "range": "± 496231",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 509,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 701,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1288,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1094,
            "range": "± 16",
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
          "id": "0c873bbb27e63173f533787dc862771a7ce7fe22",
          "message": "Merge pull request #230 from syangkkim/claude/next-issue-0UWUJ\n\nfeat: add --group-by and --agg flags for convert/view",
          "timestamp": "2026-03-26T19:23:21+09:00",
          "tree_id": "ccf96e5269502c380ef9acd79783de1e2e7425ca",
          "url": "https://github.com/syangkkim/dkit/commit/0c873bbb27e63173f533787dc862771a7ce7fe22"
        },
        "date": 1774521132158,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 1777872,
            "range": "± 56886",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 21706180,
            "range": "± 794594",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2113482,
            "range": "± 3923",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 28353445,
            "range": "± 669146",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 1793137,
            "range": "± 14048",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 18912662,
            "range": "± 283474",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 1959012,
            "range": "± 11780",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 21518929,
            "range": "± 419584",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 829678,
            "range": "± 3788",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 746448,
            "range": "± 3952",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 666520,
            "range": "± 2368",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1103197,
            "range": "± 61553",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 15583727,
            "range": "± 356577",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 877256,
            "range": "± 2274",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 12789811,
            "range": "± 302419",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1215600,
            "range": "± 16563",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 12983486,
            "range": "± 224342",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 746827,
            "range": "± 2441",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 7694573,
            "range": "± 160025",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 744009,
            "range": "± 2335",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 3714957,
            "range": "± 8168",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 442868,
            "range": "± 1683",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2222552,
            "range": "± 4398",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 494503,
            "range": "± 1895",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 10507019,
            "range": "± 376960",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1680308,
            "range": "± 24178",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 28725686,
            "range": "± 1295793",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1136033,
            "range": "± 4647",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 27989060,
            "range": "± 1180604",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 608143,
            "range": "± 1051",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 9836151,
            "range": "± 385481",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 827442,
            "range": "± 2235",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 10828933,
            "range": "± 451211",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1556546,
            "range": "± 7420",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 23696743,
            "range": "± 869185",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 490,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 626,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1222,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 979,
            "range": "± 1",
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
          "id": "50624575cd9605a36f18c7c836493b32db0ae90e",
          "message": "Merge pull request #231 from syangkkim/claude/ci-miri-benchmarks-1alBx\n\nDisable pull request triggers for miri and benchmark workflows",
          "timestamp": "2026-03-26T19:57:05+09:00",
          "tree_id": "7f031a8d3b42513bece86f195c7edc9ed6f51b57",
          "url": "https://github.com/syangkkim/dkit/commit/50624575cd9605a36f18c7c836493b32db0ae90e"
        },
        "date": 1774523153468,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2189046,
            "range": "± 89414",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22221741,
            "range": "± 521417",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2282247,
            "range": "± 5536",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 29128803,
            "range": "± 608489",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2163956,
            "range": "± 15254",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21984530,
            "range": "± 340926",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2230682,
            "range": "± 5546",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22376749,
            "range": "± 203936",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 929282,
            "range": "± 9986",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 794706,
            "range": "± 2432",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 808531,
            "range": "± 1862",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1348180,
            "range": "± 29285",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 18033851,
            "range": "± 143092",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 966378,
            "range": "± 2504",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 15050906,
            "range": "± 610327",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1351001,
            "range": "± 31933",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13489354,
            "range": "± 205308",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 820741,
            "range": "± 1622",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8263385,
            "range": "± 56387",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 870872,
            "range": "± 10768",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4304480,
            "range": "± 23257",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 496417,
            "range": "± 5994",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2482627,
            "range": "± 18661",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 603699,
            "range": "± 13851",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 12392130,
            "range": "± 375980",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1791704,
            "range": "± 66028",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 25194435,
            "range": "± 1197496",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1173483,
            "range": "± 29775",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 27134487,
            "range": "± 1855423",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 599748,
            "range": "± 2538",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 11380979,
            "range": "± 337683",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 873211,
            "range": "± 3922",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9114463,
            "range": "± 161238",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1705950,
            "range": "± 11769",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 24075300,
            "range": "± 1383949",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 506,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 697,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1301,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1066,
            "range": "± 2",
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
          "id": "ac7a499bddf735435dfab7f5d79db7a716f361cb",
          "message": "Merge pull request #232 from syangkkim/claude/next-issue-Jt1LJ\n\nfeat: add .env format Reader/Writer",
          "timestamp": "2026-03-26T20:16:18+09:00",
          "tree_id": "8f8709636bb6f93172f785bcfbb7d5bd15128e67",
          "url": "https://github.com/syangkkim/dkit/commit/ac7a499bddf735435dfab7f5d79db7a716f361cb"
        },
        "date": 1774524315343,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2155025,
            "range": "± 12358",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22035836,
            "range": "± 258941",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2252455,
            "range": "± 168672",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 29194280,
            "range": "± 561866",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2136314,
            "range": "± 13923",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21632903,
            "range": "± 612918",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2218472,
            "range": "± 11654",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22185706,
            "range": "± 232794",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 892389,
            "range": "± 27185",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 788988,
            "range": "± 16168",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 795962,
            "range": "± 13576",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1369408,
            "range": "± 29588",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 18308034,
            "range": "± 244802",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 947779,
            "range": "± 4735",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 14643892,
            "range": "± 229754",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1342311,
            "range": "± 31986",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13364328,
            "range": "± 101663",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 791637,
            "range": "± 3390",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 7861036,
            "range": "± 50225",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 870072,
            "range": "± 18954",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4319494,
            "range": "± 94642",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 491769,
            "range": "± 2199",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2485498,
            "range": "± 11814",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 593781,
            "range": "± 3562",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 12552493,
            "range": "± 368335",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1787175,
            "range": "± 25518",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 24537475,
            "range": "± 1106000",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1157634,
            "range": "± 33790",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 27087382,
            "range": "± 1517557",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 597711,
            "range": "± 2319",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 11203850,
            "range": "± 328386",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 880603,
            "range": "± 8443",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9193212,
            "range": "± 376079",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1669805,
            "range": "± 22175",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 24803055,
            "range": "± 1088722",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 512,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 722,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1306,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1108,
            "range": "± 7",
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
          "id": "7f1f0ec6992ebfa3d1d06aafabd7c4fc5c816e84",
          "message": "Merge pull request #233 from syangkkim/claude/next-issue-fBY3L\n\nfeat: add --dry-run flag for convert command",
          "timestamp": "2026-03-26T20:26:37+09:00",
          "tree_id": "89bbc111a69657372152511a066ea4f563853655",
          "url": "https://github.com/syangkkim/dkit/commit/7f1f0ec6992ebfa3d1d06aafabd7c4fc5c816e84"
        },
        "date": 1774524940350,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 1791760,
            "range": "± 52069",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 20634081,
            "range": "± 366933",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2114012,
            "range": "± 56084",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 26604701,
            "range": "± 429042",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 1794437,
            "range": "± 16012",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 18719544,
            "range": "± 219418",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 1959846,
            "range": "± 11193",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 20354070,
            "range": "± 232932",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 833186,
            "range": "± 3435",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 710501,
            "range": "± 2287",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 663903,
            "range": "± 5479",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1114968,
            "range": "± 22744",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 15218813,
            "range": "± 294804",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 880670,
            "range": "± 2272",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 12182850,
            "range": "± 349953",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1221725,
            "range": "± 12997",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 12906737,
            "range": "± 170144",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 720820,
            "range": "± 12677",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 7511279,
            "range": "± 99547",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 740039,
            "range": "± 4590",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 3741536,
            "range": "± 10951",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 444844,
            "range": "± 1145",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2230973,
            "range": "± 6513",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 499777,
            "range": "± 2534",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 10485810,
            "range": "± 337124",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1684142,
            "range": "± 7370",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 24790540,
            "range": "± 1073599",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1126480,
            "range": "± 10378",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 26231979,
            "range": "± 958809",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 612993,
            "range": "± 4048",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 9723206,
            "range": "± 343345",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 834538,
            "range": "± 8648",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9456959,
            "range": "± 283342",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1538051,
            "range": "± 29392",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 23048007,
            "range": "± 680485",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 491,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 638,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1201,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 977,
            "range": "± 3",
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
          "id": "b60d0dad662e61d3ba0806d05b281eee13db2410",
          "message": "Merge pull request #234 from syangkkim/claude/next-issue-eQmhj\n\nfeat: add --output-format flag for stats/schema (JSON/YAML output)",
          "timestamp": "2026-03-26T20:37:09+09:00",
          "tree_id": "37a33f88677441ff7025d42ff265e07bb697d9c5",
          "url": "https://github.com/syangkkim/dkit/commit/b60d0dad662e61d3ba0806d05b281eee13db2410"
        },
        "date": 1774525555933,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2193072,
            "range": "± 148337",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22383657,
            "range": "± 238055",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2258576,
            "range": "± 12603",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 28901989,
            "range": "± 876215",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2189569,
            "range": "± 7497",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 22677040,
            "range": "± 1304222",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2222794,
            "range": "± 12568",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22426971,
            "range": "± 776434",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 895092,
            "range": "± 16669",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 790439,
            "range": "± 4072",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 783660,
            "range": "± 16719",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1352359,
            "range": "± 24737",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17905009,
            "range": "± 141443",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 950630,
            "range": "± 2393",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 14333347,
            "range": "± 86289",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1339983,
            "range": "± 14936",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13370031,
            "range": "± 115300",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 786762,
            "range": "± 3899",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 7883347,
            "range": "± 44090",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 858029,
            "range": "± 4561",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4283514,
            "range": "± 12434",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 490098,
            "range": "± 1332",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2468726,
            "range": "± 85628",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 596966,
            "range": "± 11836",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 13294215,
            "range": "± 736118",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1945726,
            "range": "± 28345",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 29049702,
            "range": "± 899066",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1185093,
            "range": "± 30844",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 31294053,
            "range": "± 1799503",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 603898,
            "range": "± 7594",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 12842845,
            "range": "± 515025",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 883957,
            "range": "± 12737",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9585754,
            "range": "± 360112",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1692058,
            "range": "± 63860",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 27500425,
            "range": "± 987513",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 526,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 721,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1361,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1122,
            "range": "± 3",
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
          "id": "9a6027b85ed7439b8177d967718b1af1774f99b7",
          "message": "Merge pull request #235 from syangkkim/claude/next-issue-pCQBC\n\ntest: add v1.1.0 integration tests and update documentation",
          "timestamp": "2026-03-26T20:56:01+09:00",
          "tree_id": "21482b616985841995a77971b5ad62146066456b",
          "url": "https://github.com/syangkkim/dkit/commit/9a6027b85ed7439b8177d967718b1af1774f99b7"
        },
        "date": 1774526696601,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2148591,
            "range": "± 69473",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22049310,
            "range": "± 365563",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2255477,
            "range": "± 16679",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 31654430,
            "range": "± 388539",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2121993,
            "range": "± 5835",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21415417,
            "range": "± 237557",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2226393,
            "range": "± 5582",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22219900,
            "range": "± 56053",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 898004,
            "range": "± 10503",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 796067,
            "range": "± 2435",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 800499,
            "range": "± 6936",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1415399,
            "range": "± 19242",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 18709427,
            "range": "± 238336",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 955343,
            "range": "± 9489",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 14969289,
            "range": "± 648250",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1338695,
            "range": "± 9367",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13345428,
            "range": "± 358347",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 795963,
            "range": "± 11492",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8023690,
            "range": "± 62246",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 845938,
            "range": "± 1954",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4205376,
            "range": "± 11870",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 496895,
            "range": "± 1515",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2495797,
            "range": "± 91515",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 594847,
            "range": "± 1364",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 12530528,
            "range": "± 369935",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1790881,
            "range": "± 9626",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 24994041,
            "range": "± 1062287",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1186016,
            "range": "± 18992",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 27105349,
            "range": "± 997748",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 597285,
            "range": "± 1792",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 11724180,
            "range": "± 307771",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 874369,
            "range": "± 2721",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9291748,
            "range": "± 194084",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1702529,
            "range": "± 9496",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 24945949,
            "range": "± 1257447",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 534,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 726,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1342,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1133,
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
          "id": "e26feea183f8f5b9a9da4a9633f75bc6dc90835b",
          "message": "Merge pull request #236 from syangkkim/claude/next-issue-UYcO8\n\nchore: v1.1.0 릴리스 준비",
          "timestamp": "2026-03-26T21:03:46+09:00",
          "tree_id": "f0f77b09a13d1638ddcd353142ee9e0fd7babca7",
          "url": "https://github.com/syangkkim/dkit/commit/e26feea183f8f5b9a9da4a9633f75bc6dc90835b"
        },
        "date": 1774527163215,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2193508,
            "range": "± 42065",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22182492,
            "range": "± 133780",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2290361,
            "range": "± 81829",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 28189755,
            "range": "± 385714",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2165460,
            "range": "± 6020",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21787744,
            "range": "± 64331",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2218078,
            "range": "± 52505",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22172921,
            "range": "± 313424",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 903546,
            "range": "± 5595",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 795029,
            "range": "± 7266",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 786436,
            "range": "± 2305",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1369651,
            "range": "± 19420",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 18182894,
            "range": "± 437042",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 968084,
            "range": "± 3553",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 14612923,
            "range": "± 269893",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1341852,
            "range": "± 9852",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13403812,
            "range": "± 127352",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 790721,
            "range": "± 2436",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 7955545,
            "range": "± 51668",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 845432,
            "range": "± 4433",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4223022,
            "range": "± 70710",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 483537,
            "range": "± 7449",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2431009,
            "range": "± 4832",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 603399,
            "range": "± 12534",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 12026545,
            "range": "± 178399",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1790453,
            "range": "± 22536",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 23930236,
            "range": "± 388651",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1196443,
            "range": "± 19907",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 24919267,
            "range": "± 659009",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 604975,
            "range": "± 1379",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 11004636,
            "range": "± 174146",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 877973,
            "range": "± 16544",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 8993095,
            "range": "± 74743",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1696730,
            "range": "± 18838",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 23530181,
            "range": "± 330905",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 511,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 698,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1287,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1062,
            "range": "± 18",
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
          "id": "1736e76ff20f22cd85b1aceec31cd79878c4e0bf",
          "message": "Merge pull request #237 from syangkkim/claude/next-issue-msw1l",
          "timestamp": "2026-03-27T08:37:50+09:00",
          "tree_id": "1c93ea9ab4269a1222597b270741cd38bbb19aac",
          "url": "https://github.com/syangkkim/dkit/commit/1736e76ff20f22cd85b1aceec31cd79878c4e0bf"
        },
        "date": 1774568829657,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2166973,
            "range": "± 19812",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 21933499,
            "range": "± 160871",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2294410,
            "range": "± 107658",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 31604180,
            "range": "± 1009539",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2129122,
            "range": "± 5721",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21810396,
            "range": "± 362510",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2213420,
            "range": "± 5740",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22220341,
            "range": "± 81038",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 899200,
            "range": "± 7797",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 796256,
            "range": "± 3013",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 794708,
            "range": "± 12186",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1357572,
            "range": "± 20752",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 18262223,
            "range": "± 284169",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 949796,
            "range": "± 12021",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 15072608,
            "range": "± 357594",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1353257,
            "range": "± 20185",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13527693,
            "range": "± 274176",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 802786,
            "range": "± 4343",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8047137,
            "range": "± 20753",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 865429,
            "range": "± 2103",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4295467,
            "range": "± 9703",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 501540,
            "range": "± 2389",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2515409,
            "range": "± 6410",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 599275,
            "range": "± 3257",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 12556175,
            "range": "± 373526",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1785980,
            "range": "± 139564",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 26364399,
            "range": "± 1403991",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1166842,
            "range": "± 23227",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 27037900,
            "range": "± 1302311",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 607550,
            "range": "± 2003",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 11380915,
            "range": "± 328093",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 870442,
            "range": "± 3251",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9363874,
            "range": "± 279381",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1684771,
            "range": "± 24124",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 25133814,
            "range": "± 1186456",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 509,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 681,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1303,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1054,
            "range": "± 9",
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
          "id": "da2f3b895f1f63c605867e3fbad3c61d2feb22cb",
          "message": "Merge pull request #238 from syangkkim/claude/next-issue-6AHmU",
          "timestamp": "2026-03-27T08:54:51+09:00",
          "tree_id": "4a168045f844603769d77e7439c6bd085e51cb66",
          "url": "https://github.com/syangkkim/dkit/commit/da2f3b895f1f63c605867e3fbad3c61d2feb22cb"
        },
        "date": 1774569821502,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2185652,
            "range": "± 125271",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22083323,
            "range": "± 203862",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2332018,
            "range": "± 23912",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 28217404,
            "range": "± 379983",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2142605,
            "range": "± 5898",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21604837,
            "range": "± 412878",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2218984,
            "range": "± 14995",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22249821,
            "range": "± 73560",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 904976,
            "range": "± 6642",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 804236,
            "range": "± 4013",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 776490,
            "range": "± 8599",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1357955,
            "range": "± 6263",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17424383,
            "range": "± 109177",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 945414,
            "range": "± 2677",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 13822302,
            "range": "± 161879",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1348730,
            "range": "± 20008",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13466579,
            "range": "± 91271",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 805877,
            "range": "± 2832",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 7951821,
            "range": "± 61007",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 855552,
            "range": "± 22167",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4248509,
            "range": "± 18722",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 500782,
            "range": "± 2449",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2510312,
            "range": "± 7145",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 592018,
            "range": "± 2208",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 11413938,
            "range": "± 227090",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1797746,
            "range": "± 26323",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 23600913,
            "range": "± 448988",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1184389,
            "range": "± 13902",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 24876977,
            "range": "± 825329",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 605426,
            "range": "± 6588",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10516478,
            "range": "± 137216",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 881296,
            "range": "± 4087",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9012846,
            "range": "± 175676",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1685614,
            "range": "± 12781",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 22351543,
            "range": "± 630313",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 495,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 671,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1267,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1047,
            "range": "± 15",
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
          "id": "7ac87654272a2ed3bedbf22bd120eca72f1df4d3",
          "message": "Merge pull request #239 from syangkkim/claude/next-issue-Cwv2S\n\nfeat: add --map flag for value transformation",
          "timestamp": "2026-03-27T09:31:25+09:00",
          "tree_id": "70848d19528fb49ebf939614de6cc6d7278b85e1",
          "url": "https://github.com/syangkkim/dkit/commit/7ac87654272a2ed3bedbf22bd120eca72f1df4d3"
        },
        "date": 1774572020200,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2234719,
            "range": "± 10654",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22560720,
            "range": "± 436487",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2318429,
            "range": "± 93101",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 29246188,
            "range": "± 285324",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2196446,
            "range": "± 18221",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 22436898,
            "range": "± 399766",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2264904,
            "range": "± 10882",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22228810,
            "range": "± 125487",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 916794,
            "range": "± 12097",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 797100,
            "range": "± 3903",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 787380,
            "range": "± 12982",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1361266,
            "range": "± 36350",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 18007772,
            "range": "± 61069",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 953799,
            "range": "± 5010",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 14368699,
            "range": "± 66612",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1346462,
            "range": "± 11658",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13372960,
            "range": "± 319992",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 793390,
            "range": "± 29574",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 7953248,
            "range": "± 449668",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 875001,
            "range": "± 18894",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4351801,
            "range": "± 20010",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 493619,
            "range": "± 2398",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2486041,
            "range": "± 9180",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 588476,
            "range": "± 16877",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 12449483,
            "range": "± 318347",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1916895,
            "range": "± 61555",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 25401893,
            "range": "± 1459418",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1169726,
            "range": "± 20803",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 25963102,
            "range": "± 1123736",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 598023,
            "range": "± 4036",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 11331183,
            "range": "± 333742",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 873438,
            "range": "± 20207",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9028649,
            "range": "± 191629",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1687168,
            "range": "± 17840",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 17983487,
            "range": "± 324050",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 524,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 701,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1366,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1093,
            "range": "± 14",
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
          "id": "7f114182958d1dd29c929a3cbf4e3a0403623afd",
          "message": "Merge pull request #240 from syangkkim/claude/next-issue-4mZl1\n\nfeat: add array slicing & wildcard in query paths",
          "timestamp": "2026-03-27T10:04:21+09:00",
          "tree_id": "a172327d3ac660f6a40b352f529381f375cd4c04",
          "url": "https://github.com/syangkkim/dkit/commit/7f114182958d1dd29c929a3cbf4e3a0403623afd"
        },
        "date": 1774573995296,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2185465,
            "range": "± 104010",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22098820,
            "range": "± 339469",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2300996,
            "range": "± 51629",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 27660396,
            "range": "± 610624",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2161362,
            "range": "± 6700",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21621387,
            "range": "± 80540",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2234522,
            "range": "± 5050",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22342499,
            "range": "± 105675",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 924646,
            "range": "± 2896",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 808181,
            "range": "± 2918",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 792288,
            "range": "± 3050",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1366800,
            "range": "± 9908",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17388815,
            "range": "± 262316",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 948631,
            "range": "± 4122",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 13678289,
            "range": "± 119233",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1359522,
            "range": "± 24116",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13515556,
            "range": "± 107538",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 784660,
            "range": "± 3451",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8014559,
            "range": "± 64040",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 856976,
            "range": "± 6101",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4245953,
            "range": "± 17327",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 496246,
            "range": "± 13637",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2498582,
            "range": "± 4527",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 595397,
            "range": "± 6853",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 11202006,
            "range": "± 225962",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1785239,
            "range": "± 20007",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 23120315,
            "range": "± 264639",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1192102,
            "range": "± 7542",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 26377671,
            "range": "± 1512598",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 604564,
            "range": "± 3016",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10687256,
            "range": "± 264795",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 883615,
            "range": "± 5399",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 8898330,
            "range": "± 173329",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1698212,
            "range": "± 11796",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 26542837,
            "range": "± 1750620",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 515,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 685,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1330,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1077,
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
          "id": "7de97da909c7cd9991836011f0b570c0eb28fc38",
          "message": "Merge pull request #241 from syangkkim/claude/next-issue-mIizd\n\nfeat: add .ini/.cfg format Reader/Writer",
          "timestamp": "2026-03-27T10:28:57+09:00",
          "tree_id": "c3688882a018bc9911e1aaa5daef09b0b7f21a8c",
          "url": "https://github.com/syangkkim/dkit/commit/7de97da909c7cd9991836011f0b570c0eb28fc38"
        },
        "date": 1774575476733,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2181118,
            "range": "± 74409",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 21959439,
            "range": "± 67314",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2306873,
            "range": "± 12694",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 30684099,
            "range": "± 271168",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2143254,
            "range": "± 15968",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21597896,
            "range": "± 85835",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2225681,
            "range": "± 9729",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22205243,
            "range": "± 64564",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 908547,
            "range": "± 9232",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 819844,
            "range": "± 2094",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 798577,
            "range": "± 8092",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1354201,
            "range": "± 15888",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17873327,
            "range": "± 44328",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 996787,
            "range": "± 2724",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 14715687,
            "range": "± 56733",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1347862,
            "range": "± 16099",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13404472,
            "range": "± 39763",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 825938,
            "range": "± 4455",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8249280,
            "range": "± 50066",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 862104,
            "range": "± 2575",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4300675,
            "range": "± 17082",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 496086,
            "range": "± 2900",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2484174,
            "range": "± 32941",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 592325,
            "range": "± 4476",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 11678357,
            "range": "± 172850",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1796681,
            "range": "± 20013",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 23721269,
            "range": "± 82503",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1183203,
            "range": "± 23310",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 25071249,
            "range": "± 140166",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 608749,
            "range": "± 1968",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10954489,
            "range": "± 171323",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 876487,
            "range": "± 1998",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 8959870,
            "range": "± 37310",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1691554,
            "range": "± 8247",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 22436924,
            "range": "± 144995",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 502,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 695,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1324,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1061,
            "range": "± 19",
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
          "id": "630a61514d10ad6be1707e97beea98cc26a70708",
          "message": "Merge pull request #242 from syangkkim/claude/next-issue-z2Bdi\n\nfeat: add .properties format Reader/Writer",
          "timestamp": "2026-03-27T10:45:12+09:00",
          "tree_id": "16d79062df5d879c4865f7e7c686f2d4bf68286b",
          "url": "https://github.com/syangkkim/dkit/commit/630a61514d10ad6be1707e97beea98cc26a70708"
        },
        "date": 1774576438755,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2187367,
            "range": "± 11637",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 25802843,
            "range": "± 478868",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2303645,
            "range": "± 94460",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 30983736,
            "range": "± 526565",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2233648,
            "range": "± 15108",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 24854867,
            "range": "± 395711",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2389277,
            "range": "± 32698",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 23151589,
            "range": "± 176321",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 900636,
            "range": "± 6792",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 804113,
            "range": "± 4725",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 792807,
            "range": "± 2753",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1351032,
            "range": "± 25286",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17309083,
            "range": "± 158594",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 959547,
            "range": "± 5953",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 15006558,
            "range": "± 545988",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1356518,
            "range": "± 25624",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 14833826,
            "range": "± 387306",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 806639,
            "range": "± 7389",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8120907,
            "range": "± 94322",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 866949,
            "range": "± 18452",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4270998,
            "range": "± 97922",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 498922,
            "range": "± 6076",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2516805,
            "range": "± 8414",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 598863,
            "range": "± 4615",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 13660780,
            "range": "± 1219231",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1807479,
            "range": "± 10896",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 25139017,
            "range": "± 1572120",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1185981,
            "range": "± 49058",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 27207443,
            "range": "± 2262394",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 606849,
            "range": "± 2254",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 13138382,
            "range": "± 375323",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 889500,
            "range": "± 2837",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9860899,
            "range": "± 482292",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1830440,
            "range": "± 13714",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 25894049,
            "range": "± 2417852",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 512,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 687,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1339,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1087,
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
          "id": "45066e9a2e99b7b73b710b9dec56cfc15813a49c",
          "message": "Merge pull request #243 from syangkkim/claude/next-issue-5aStS\n\nfeat: add IN / NOT IN operators for query where clause",
          "timestamp": "2026-03-27T10:57:48+09:00",
          "tree_id": "24ef3b64c3b59620cd2e05f1b3eb4d296d099356",
          "url": "https://github.com/syangkkim/dkit/commit/45066e9a2e99b7b73b710b9dec56cfc15813a49c"
        },
        "date": 1774577207506,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2208304,
            "range": "± 33843",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22220415,
            "range": "± 120998",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2303827,
            "range": "± 111830",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 30705355,
            "range": "± 967737",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2169649,
            "range": "± 9121",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 22079331,
            "range": "± 86753",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2220251,
            "range": "± 5599",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 21782492,
            "range": "± 83735",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 965247,
            "range": "± 8278",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 827177,
            "range": "± 2052",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 836063,
            "range": "± 2395",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1370979,
            "range": "± 7629",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 19497562,
            "range": "± 296706",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 990248,
            "range": "± 4817",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 15809977,
            "range": "± 56632",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1329504,
            "range": "± 21699",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13187445,
            "range": "± 56445",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 811641,
            "range": "± 5399",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8059129,
            "range": "± 30126",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 822182,
            "range": "± 2064",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4104543,
            "range": "± 17542",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 457285,
            "range": "± 1545",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2302419,
            "range": "± 11817",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 547658,
            "range": "± 6915",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 13395038,
            "range": "± 261531",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1807073,
            "range": "± 18355",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 28309928,
            "range": "± 1537824",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1244760,
            "range": "± 17838",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 31947488,
            "range": "± 1323073",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 584790,
            "range": "± 1418",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 11572195,
            "range": "± 143518",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 825099,
            "range": "± 1725",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 8621022,
            "range": "± 1309504",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1666944,
            "range": "± 25331",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 24224604,
            "range": "± 83256",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 578,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 687,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1284,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1143,
            "range": "± 3",
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
          "id": "bc4a2e177d26d6f8036d25e1788d0d13181e62c5",
          "message": "Merge pull request #244 from syangkkim/claude/next-issue-sW86S\n\nfeat: add `matches` regex operator for query where clause",
          "timestamp": "2026-03-27T11:08:09+09:00",
          "tree_id": "a4c40a966af68ee580427f45cfe2f5f0ab7cdb6d",
          "url": "https://github.com/syangkkim/dkit/commit/bc4a2e177d26d6f8036d25e1788d0d13181e62c5"
        },
        "date": 1774577823484,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2150684,
            "range": "± 55079",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 21829438,
            "range": "± 894311",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2282790,
            "range": "± 80147",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 31209015,
            "range": "± 548237",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2138117,
            "range": "± 11060",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21389851,
            "range": "± 269194",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2204158,
            "range": "± 13759",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 21965106,
            "range": "± 239925",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 903001,
            "range": "± 6182",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 800744,
            "range": "± 2661",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 787293,
            "range": "± 1508",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1374595,
            "range": "± 22475",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 18180121,
            "range": "± 236994",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 949094,
            "range": "± 13022",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 14482716,
            "range": "± 486808",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1347903,
            "range": "± 16973",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13414121,
            "range": "± 251840",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 800523,
            "range": "± 3257",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8024151,
            "range": "± 22113",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 898411,
            "range": "± 3338",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4329821,
            "range": "± 20713",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 493187,
            "range": "± 1413",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2478466,
            "range": "± 18574",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 588078,
            "range": "± 2346",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 11970117,
            "range": "± 239595",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1785086,
            "range": "± 20685",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 24507397,
            "range": "± 2047979",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1171035,
            "range": "± 9788",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 25404754,
            "range": "± 1239173",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 598523,
            "range": "± 1826",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10963254,
            "range": "± 222841",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 870222,
            "range": "± 5412",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9071171,
            "range": "± 203679",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1675758,
            "range": "± 20949",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 23253238,
            "range": "± 1240205",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 556,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 705,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1359,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1146,
            "range": "± 5",
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
          "id": "2a37e02e465e0b5fc263208098ce41c21e91e135",
          "message": "Merge pull request #245 from syangkkim/claude/next-issue-bmqii\n\nfeat: add --indent, --sort-keys, --compact JSON output options",
          "timestamp": "2026-03-27T11:20:36+09:00",
          "tree_id": "9fd71a673e5aeab7e53fa65bc2c3cb45c2c51f6f",
          "url": "https://github.com/syangkkim/dkit/commit/2a37e02e465e0b5fc263208098ce41c21e91e135"
        },
        "date": 1774578569781,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2172181,
            "range": "± 61486",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22688624,
            "range": "± 661028",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2812971,
            "range": "± 106349",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 39276327,
            "range": "± 823691",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2156370,
            "range": "± 46528",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 22095623,
            "range": "± 301224",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2233689,
            "range": "± 9850",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22195466,
            "range": "± 148194",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 1358889,
            "range": "± 27883",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 799158,
            "range": "± 3339",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 790967,
            "range": "± 10831",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1379268,
            "range": "± 35552",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 18151228,
            "range": "± 763651",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1429991,
            "range": "± 19698",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 23733964,
            "range": "± 571713",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1339903,
            "range": "± 16569",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13291329,
            "range": "± 85148",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 804647,
            "range": "± 2728",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8115714,
            "range": "± 47250",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 877873,
            "range": "± 6697",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4395281,
            "range": "± 26286",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 501841,
            "range": "± 2333",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2540931,
            "range": "± 17152",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 591047,
            "range": "± 4779",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 11347323,
            "range": "± 448263",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1787709,
            "range": "± 12348",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 24416057,
            "range": "± 1744158",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1161186,
            "range": "± 21267",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 26082844,
            "range": "± 1955177",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 603048,
            "range": "± 4841",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10673621,
            "range": "± 290174",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 873482,
            "range": "± 3509",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9152414,
            "range": "± 252423",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1701534,
            "range": "± 22049",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 24285430,
            "range": "± 1749749",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 573,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 719,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1365,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1132,
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
          "id": "446824151ccb6a9ac50eaa0f406ead7509c069df",
          "message": "Merge pull request #246 from syangkkim/claude/next-issue-iKmpS",
          "timestamp": "2026-03-27T12:13:10+09:00",
          "tree_id": "d5addfc3ae00aa8ba3bb5389492e14623e3edb8e",
          "url": "https://github.com/syangkkim/dkit/commit/446824151ccb6a9ac50eaa0f406ead7509c069df"
        },
        "date": 1774581733784,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2194110,
            "range": "± 37103",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 23356368,
            "range": "± 518321",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2820899,
            "range": "± 148497",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 40529061,
            "range": "± 414542",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2144532,
            "range": "± 8669",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21989194,
            "range": "± 595828",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2219513,
            "range": "± 10299",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22143800,
            "range": "± 117554",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 1357488,
            "range": "± 10450",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 804224,
            "range": "± 4578",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 777045,
            "range": "± 2638",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1356059,
            "range": "± 8229",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17926680,
            "range": "± 107996",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1433524,
            "range": "± 27895",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 26373280,
            "range": "± 471815",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1377978,
            "range": "± 18817",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 14009296,
            "range": "± 457793",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 807851,
            "range": "± 3841",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8015398,
            "range": "± 121341",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 864315,
            "range": "± 2941",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4332581,
            "range": "± 21019",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 502233,
            "range": "± 1948",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2526822,
            "range": "± 17829",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 597321,
            "range": "± 3063",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 12331137,
            "range": "± 245750",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1792843,
            "range": "± 18013",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 27476515,
            "range": "± 2120605",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1186034,
            "range": "± 40940",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 27061710,
            "range": "± 1459294",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 598096,
            "range": "± 2030",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10922082,
            "range": "± 493454",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 875061,
            "range": "± 4174",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 8937834,
            "range": "± 81029",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1656388,
            "range": "± 15920",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 22446437,
            "range": "± 372913",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 565,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 708,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1343,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1141,
            "range": "± 2",
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
          "id": "823016e4cb705e99939566e672d8b33ac0867336",
          "message": "Merge pull request #247 from syangkkim/claude/next-issue-vJoJb\n\ntest: add v1.2.0 integration tests and update documentation",
          "timestamp": "2026-03-27T12:30:05+09:00",
          "tree_id": "c9fce5c3806007e02bb0b52b8619303b04b989b0",
          "url": "https://github.com/syangkkim/dkit/commit/823016e4cb705e99939566e672d8b33ac0867336"
        },
        "date": 1774582744775,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2175880,
            "range": "± 146621",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 21955496,
            "range": "± 170168",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2792762,
            "range": "± 27204",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 38573351,
            "range": "± 320077",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2136435,
            "range": "± 24380",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21421595,
            "range": "± 117234",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2216570,
            "range": "± 10538",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22069753,
            "range": "± 59794",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 1359159,
            "range": "± 9023",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 795698,
            "range": "± 2819",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 775437,
            "range": "± 7209",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1362577,
            "range": "± 26130",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 18035062,
            "range": "± 143597",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1433836,
            "range": "± 11544",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 24407755,
            "range": "± 499078",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1339334,
            "range": "± 17088",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13406447,
            "range": "± 74945",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 807361,
            "range": "± 2133",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8090662,
            "range": "± 72400",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 870945,
            "range": "± 6684",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4360512,
            "range": "± 21524",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 502535,
            "range": "± 1301",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2530707,
            "range": "± 18463",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 587493,
            "range": "± 3489",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 12186209,
            "range": "± 310309",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1770043,
            "range": "± 28730",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 24346202,
            "range": "± 954561",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1160715,
            "range": "± 10516",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 25846913,
            "range": "± 1620511",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 598744,
            "range": "± 2210",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10958676,
            "range": "± 243852",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 869994,
            "range": "± 2456",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9248443,
            "range": "± 310714",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1667382,
            "range": "± 28227",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 22804409,
            "range": "± 517207",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 569,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 695,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1350,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1144,
            "range": "± 8",
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
          "id": "c3168bd3c7f06b79c742432c30dde96ddb428cf4",
          "message": "Merge pull request #248 from syangkkim/claude/next-issue-Z65Wh\n\nchore: v1.2.0 릴리스 준비",
          "timestamp": "2026-03-27T12:47:21+09:00",
          "tree_id": "99409c267595639d5cc4f06da246b8b16463f25c",
          "url": "https://github.com/syangkkim/dkit/commit/c3168bd3c7f06b79c742432c30dde96ddb428cf4"
        },
        "date": 1774583780618,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2195349,
            "range": "± 70168",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22094651,
            "range": "± 225289",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2835539,
            "range": "± 108022",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 39615004,
            "range": "± 1534864",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2181236,
            "range": "± 9102",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21752896,
            "range": "± 102919",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2209783,
            "range": "± 13013",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22155795,
            "range": "± 74748",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 1368492,
            "range": "± 5586",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 802346,
            "range": "± 11571",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 830344,
            "range": "± 2294",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1346503,
            "range": "± 6268",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17824170,
            "range": "± 425469",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1437252,
            "range": "± 9990",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 24203784,
            "range": "± 300031",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1351785,
            "range": "± 16513",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13455633,
            "range": "± 65732",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 819589,
            "range": "± 5817",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8231791,
            "range": "± 61830",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 859283,
            "range": "± 2919",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4258544,
            "range": "± 12318",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 489962,
            "range": "± 1455",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2458555,
            "range": "± 5736",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 602131,
            "range": "± 4124",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 11764963,
            "range": "± 114756",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1815173,
            "range": "± 20051",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 23894965,
            "range": "± 368438",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1176217,
            "range": "± 20526",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 25436898,
            "range": "± 424869",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 603085,
            "range": "± 3471",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10832052,
            "range": "± 106098",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 878162,
            "range": "± 2420",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 8964226,
            "range": "± 64801",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1703599,
            "range": "± 11328",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 22539958,
            "range": "± 397095",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 574,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 694,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1399,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1175,
            "range": "± 19",
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
          "id": "b542410e1225b77c832a0b8fb87f92fe276bc8ae",
          "message": "Merge pull request #249 from syangkkim/claude/next-issue-0blEh\n\nfeat: add --explode flag for unnesting array fields into rows",
          "timestamp": "2026-03-27T14:08:29+09:00",
          "tree_id": "c73a41f98681674d92255309c00de265036e7ea7",
          "url": "https://github.com/syangkkim/dkit/commit/b542410e1225b77c832a0b8fb87f92fe276bc8ae"
        },
        "date": 1774588645321,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2171100,
            "range": "± 12463",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22495381,
            "range": "± 1323482",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2773297,
            "range": "± 115088",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 40075987,
            "range": "± 635812",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2170483,
            "range": "± 13160",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 22400430,
            "range": "± 796349",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2275935,
            "range": "± 6618",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22879645,
            "range": "± 488895",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 1362513,
            "range": "± 18731",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 804870,
            "range": "± 9351",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 792934,
            "range": "± 2450",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1352244,
            "range": "± 17718",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 18099805,
            "range": "± 487832",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1436480,
            "range": "± 8909",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 25036231,
            "range": "± 872956",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1356903,
            "range": "± 7467",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13621272,
            "range": "± 315523",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 822339,
            "range": "± 11692",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8264353,
            "range": "± 59208",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 862450,
            "range": "± 3864",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4305805,
            "range": "± 21823",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 507962,
            "range": "± 1194",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2547634,
            "range": "± 8085",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 591917,
            "range": "± 6289",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 12109955,
            "range": "± 547044",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1819814,
            "range": "± 12623",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 25389108,
            "range": "± 689245",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1193428,
            "range": "± 20897",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 27331953,
            "range": "± 2066412",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 600334,
            "range": "± 4369",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 11612309,
            "range": "± 850626",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 869766,
            "range": "± 3013",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9380931,
            "range": "± 735068",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1705967,
            "range": "± 22244",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 24884373,
            "range": "± 1390340",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 579,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 705,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1408,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1212,
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
          "id": "f7f3c241021a00cd61f05c62018e6d3cbc4d527a",
          "message": "Merge pull request #250 from syangkkim/claude/next-issue-DLW0G\n\nfeat: add --pivot and --unpivot flags for data reshaping",
          "timestamp": "2026-03-27T15:19:46+09:00",
          "tree_id": "20054e3899c98ab518331c4323682e8a4ec3f9f9",
          "url": "https://github.com/syangkkim/dkit/commit/f7f3c241021a00cd61f05c62018e6d3cbc4d527a"
        },
        "date": 1774592929236,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2165488,
            "range": "± 111525",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 21905252,
            "range": "± 129004",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2750485,
            "range": "± 28394",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 38334569,
            "range": "± 366521",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2133250,
            "range": "± 34910",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21308003,
            "range": "± 181051",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2212855,
            "range": "± 14423",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22244219,
            "range": "± 79981",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 1356481,
            "range": "± 18748",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 806154,
            "range": "± 2220",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 779586,
            "range": "± 5089",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1345430,
            "range": "± 26086",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17768534,
            "range": "± 447647",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1427987,
            "range": "± 10436",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 24069761,
            "range": "± 238521",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1335335,
            "range": "± 38493",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13235908,
            "range": "± 320053",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 828444,
            "range": "± 3926",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8184122,
            "range": "± 306506",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 845303,
            "range": "± 3971",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4199945,
            "range": "± 106361",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 502489,
            "range": "± 2065",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2476441,
            "range": "± 56211",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 586116,
            "range": "± 6688",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 11843593,
            "range": "± 600281",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1777089,
            "range": "± 23644",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 23654706,
            "range": "± 253325",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1176144,
            "range": "± 19163",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 24620966,
            "range": "± 402188",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 599238,
            "range": "± 3864",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10774176,
            "range": "± 327412",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 889044,
            "range": "± 22904",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 8978908,
            "range": "± 114276",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1681592,
            "range": "± 20084",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 22455873,
            "range": "± 958021",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 581,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 693,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1333,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1152,
            "range": "± 41",
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
          "id": "0a453162a73596216392e039a5a3309c344f6ba1",
          "message": "Merge pull request #251 from syangkkim/claude/next-issue-XnX55\n\nfeat: add HCL format Reader/Writer",
          "timestamp": "2026-03-27T16:02:51+09:00",
          "tree_id": "5318a2df6dcf72a1143a9cedab0ac8008d1236d5",
          "url": "https://github.com/syangkkim/dkit/commit/0a453162a73596216392e039a5a3309c344f6ba1"
        },
        "date": 1774595510205,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2168897,
            "range": "± 86735",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22198057,
            "range": "± 1127378",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2796593,
            "range": "± 103925",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 39471736,
            "range": "± 440282",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2185378,
            "range": "± 11277",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21883858,
            "range": "± 577816",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2234418,
            "range": "± 7058",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22269642,
            "range": "± 158012",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 1393955,
            "range": "± 19812",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 808375,
            "range": "± 3553",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 848759,
            "range": "± 17253",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1358406,
            "range": "± 18588",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 18162037,
            "range": "± 176523",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1428860,
            "range": "± 21174",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 24412234,
            "range": "± 322719",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1343902,
            "range": "± 18028",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13527053,
            "range": "± 91789",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 809941,
            "range": "± 2017",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8037617,
            "range": "± 31094",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 854653,
            "range": "± 2118",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4352802,
            "range": "± 9616",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 497434,
            "range": "± 1812",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2504142,
            "range": "± 16574",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 585431,
            "range": "± 2432",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 12263928,
            "range": "± 311549",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1768608,
            "range": "± 21342",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 25859843,
            "range": "± 1154247",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1170175,
            "range": "± 28308",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 28210403,
            "range": "± 1681009",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 599667,
            "range": "± 3726",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 12103606,
            "range": "± 273573",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 897171,
            "range": "± 5383",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 10016505,
            "range": "± 459386",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1741693,
            "range": "± 25082",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 27184658,
            "range": "± 1478972",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 572,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 685,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1357,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1149,
            "range": "± 9",
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
          "id": "d49eb392a750437176343e1765a49c48da88de84",
          "message": "Merge pull request #252 from syangkkim/claude/next-issue-LfA1I\n\nfeat: add .plist (macOS Property List) format Reader/Writer",
          "timestamp": "2026-03-27T16:57:26+09:00",
          "tree_id": "f2352b7560db98a8130fa0e49168ff6b27993c57",
          "url": "https://github.com/syangkkim/dkit/commit/d49eb392a750437176343e1765a49c48da88de84"
        },
        "date": 1774598783261,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2169207,
            "range": "± 19607",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 21843790,
            "range": "± 115376",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2788352,
            "range": "± 147991",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 39686672,
            "range": "± 516577",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2167272,
            "range": "± 55991",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21610469,
            "range": "± 449308",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2209594,
            "range": "± 8295",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22201125,
            "range": "± 586279",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 1413725,
            "range": "± 22776",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 800128,
            "range": "± 2282",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 811076,
            "range": "± 2668",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1370985,
            "range": "± 24233",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 18567988,
            "range": "± 668636",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1441375,
            "range": "± 30349",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 25412182,
            "range": "± 422830",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1351069,
            "range": "± 10820",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13703122,
            "range": "± 199336",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 809492,
            "range": "± 3827",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8244586,
            "range": "± 78733",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 876309,
            "range": "± 4827",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4358018,
            "range": "± 12279",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 509625,
            "range": "± 8167",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2566825,
            "range": "± 16954",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 590518,
            "range": "± 8189",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 12441560,
            "range": "± 450549",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1781422,
            "range": "± 10766",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 27500807,
            "range": "± 1156638",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1184715,
            "range": "± 26516",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 28087615,
            "range": "± 1388864",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 614116,
            "range": "± 22367",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 11685770,
            "range": "± 585932",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 900520,
            "range": "± 4411",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9810843,
            "range": "± 380904",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1705204,
            "range": "± 15659",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 24603675,
            "range": "± 1161408",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 580,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 715,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1371,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1178,
            "range": "± 3",
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
          "id": "ffbc1b52e35db5b3793f91eb21bd5c3c129cb62a",
          "message": "Merge pull request #253 from syangkkim/claude/next-issue-BsVY1\n\nfeat: add recursive descent (`..`) operator for deep key search",
          "timestamp": "2026-03-27T17:48:00+09:00",
          "tree_id": "f4ef0f5b366ece6a6542ab182e0895a59577b130",
          "url": "https://github.com/syangkkim/dkit/commit/ffbc1b52e35db5b3793f91eb21bd5c3c129cb62a"
        },
        "date": 1774601835198,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2174747,
            "range": "± 59190",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22362117,
            "range": "± 848986",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2806077,
            "range": "± 11341",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 39842047,
            "range": "± 400192",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2155811,
            "range": "± 13021",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 23270853,
            "range": "± 584298",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2215057,
            "range": "± 89006",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22334704,
            "range": "± 133979",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 1387223,
            "range": "± 8710",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 797758,
            "range": "± 3756",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 819097,
            "range": "± 20735",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1358557,
            "range": "± 17564",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17452680,
            "range": "± 99934",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1443223,
            "range": "± 7606",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 23721006,
            "range": "± 752530",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1367733,
            "range": "± 10081",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13683140,
            "range": "± 66833",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 817717,
            "range": "± 3405",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8148376,
            "range": "± 50774",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 851312,
            "range": "± 7392",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4274544,
            "range": "± 101878",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 495894,
            "range": "± 1304",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2502688,
            "range": "± 9962",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 588960,
            "range": "± 11824",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 11227527,
            "range": "± 239060",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1778535,
            "range": "± 15514",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 23838172,
            "range": "± 1018337",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1174558,
            "range": "± 20078",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 24722790,
            "range": "± 399815",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 605118,
            "range": "± 2557",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10367211,
            "range": "± 127193",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 924060,
            "range": "± 2921",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9396132,
            "range": "± 96915",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1689134,
            "range": "± 16048",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 22055057,
            "range": "± 371970",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 568,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 688,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1337,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1158,
            "range": "± 2",
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
          "id": "fce0b1cac48867eb2cfc6e156b21842e6c9ba93f",
          "message": "Merge pull request #254 from syangkkim/claude/next-issue-afrG7\n\nfeat: add conditional expressions (if/then/else, case/when) to query",
          "timestamp": "2026-03-27T18:35:08+09:00",
          "tree_id": "0c2f6721ec76d249918b01d55895d20b1c208f06",
          "url": "https://github.com/syangkkim/dkit/commit/fce0b1cac48867eb2cfc6e156b21842e6c9ba93f"
        },
        "date": 1774604647195,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 1768995,
            "range": "± 31975",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 20006989,
            "range": "± 77767",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2708334,
            "range": "± 24715",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 36108624,
            "range": "± 1057238",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 1802612,
            "range": "± 21152",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 18806886,
            "range": "± 55139",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 1939698,
            "range": "± 21538",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 21102378,
            "range": "± 275636",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 1337102,
            "range": "± 27305",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 723161,
            "range": "± 5826",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 665618,
            "range": "± 1547",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1103484,
            "range": "± 51652",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 14621855,
            "range": "± 36463",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1336050,
            "range": "± 32082",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 19298851,
            "range": "± 342995",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1209413,
            "range": "± 15123",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 12481421,
            "range": "± 62626",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 744914,
            "range": "± 1326",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 7468652,
            "range": "± 15080",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 738705,
            "range": "± 1060",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 3714286,
            "range": "± 6139",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 441381,
            "range": "± 1208",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2218940,
            "range": "± 3175",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 467309,
            "range": "± 1948",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 9425324,
            "range": "± 147450",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1634435,
            "range": "± 19086",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 21484327,
            "range": "± 212708",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1110619,
            "range": "± 4268",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 22950991,
            "range": "± 328951",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 604011,
            "range": "± 16631",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 8859777,
            "range": "± 75656",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 832519,
            "range": "± 2267",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 8972736,
            "range": "± 168203",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1504527,
            "range": "± 18439",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 20288884,
            "range": "± 175924",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 535,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 620,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1316,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1015,
            "range": "± 1",
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
          "id": "aa3976afadcb9d5473e79d97b7f78c4af417be84",
          "message": "Merge pull request #255 from syangkkim/claude/next-issue-S4MLC\n\nfeat: add statistical aggregate functions (median, percentile, stddev, variance, mode, group_concat)",
          "timestamp": "2026-03-27T18:55:52+09:00",
          "tree_id": "739fc5c59969402ebb5a5c5b73e925985b752066",
          "url": "https://github.com/syangkkim/dkit/commit/aa3976afadcb9d5473e79d97b7f78c4af417be84"
        },
        "date": 1774605893565,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2173985,
            "range": "± 39546",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 21905926,
            "range": "± 124618",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2789136,
            "range": "± 16614",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 39889022,
            "range": "± 1419033",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2159760,
            "range": "± 14853",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21684444,
            "range": "± 129640",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2247666,
            "range": "± 14729",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 24944004,
            "range": "± 540562",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 1394955,
            "range": "± 9487",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 810287,
            "range": "± 2592",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 805648,
            "range": "± 3365",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1347855,
            "range": "± 98499",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17732434,
            "range": "± 107697",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1441491,
            "range": "± 522307",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 24054506,
            "range": "± 226000",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1349136,
            "range": "± 22698",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13424160,
            "range": "± 52900",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 823138,
            "range": "± 9641",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8109040,
            "range": "± 181431",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 848471,
            "range": "± 3452",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4249162,
            "range": "± 17884",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 497085,
            "range": "± 3065",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2503431,
            "range": "± 6234",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 589152,
            "range": "± 2596",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 11668319,
            "range": "± 102891",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1788004,
            "range": "± 44434",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 23692934,
            "range": "± 166812",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1173178,
            "range": "± 18817",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 25303709,
            "range": "± 617858",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 599184,
            "range": "± 3355",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10996094,
            "range": "± 102400",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 886390,
            "range": "± 3173",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9138284,
            "range": "± 268511",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1661289,
            "range": "± 16756",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 22543659,
            "range": "± 844141",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 575,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 685,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1475,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1155,
            "range": "± 11",
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
          "id": "81e56675b8c4352a3c8af960756c6a93147d1af8",
          "message": "Merge pull request #256 from syangkkim/claude/next-issue-TFbj9\n\ntest: add v1.3.0 integration tests and update documentation",
          "timestamp": "2026-03-27T19:18:47+09:00",
          "tree_id": "4f553672da64938f70a16d841820624808d9ade7",
          "url": "https://github.com/syangkkim/dkit/commit/81e56675b8c4352a3c8af960756c6a93147d1af8"
        },
        "date": 1774607261496,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2156410,
            "range": "± 12702",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 21845651,
            "range": "± 396188",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2781883,
            "range": "± 25564",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 39286984,
            "range": "± 1198211",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2139752,
            "range": "± 57393",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21676156,
            "range": "± 602921",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2226029,
            "range": "± 14476",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22280685,
            "range": "± 292599",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 1396763,
            "range": "± 27373",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 800022,
            "range": "± 3733",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 787761,
            "range": "± 9066",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1356201,
            "range": "± 21985",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17492250,
            "range": "± 329402",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1436248,
            "range": "± 21967",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 23348705,
            "range": "± 477964",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1342546,
            "range": "± 8194",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13404718,
            "range": "± 98803",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 817957,
            "range": "± 1895",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8083028,
            "range": "± 54830",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 857110,
            "range": "± 6010",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4273962,
            "range": "± 17829",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 497855,
            "range": "± 4554",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2508085,
            "range": "± 7710",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 607962,
            "range": "± 2212",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 11541565,
            "range": "± 406565",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1797050,
            "range": "± 37136",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 24069659,
            "range": "± 1247873",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1168490,
            "range": "± 26490",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 25546827,
            "range": "± 1414150",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 599151,
            "range": "± 4822",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10780194,
            "range": "± 316732",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 891983,
            "range": "± 4340",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9201073,
            "range": "± 237440",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1702242,
            "range": "± 32246",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 24129090,
            "range": "± 1254911",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 560,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 686,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1468,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1142,
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
          "id": "b9ade709e604119e8946b0fe60db55e888ba3e33",
          "message": "Merge pull request #257 from syangkkim/claude/next-issue-X8Fn7\n\nchore: v1.3.0 릴리스 준비",
          "timestamp": "2026-03-27T19:27:55+09:00",
          "tree_id": "09d7946f571337bd73db46ea41edaf6cfbc9d899",
          "url": "https://github.com/syangkkim/dkit/commit/b9ade709e604119e8946b0fe60db55e888ba3e33"
        },
        "date": 1774607809419,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2199745,
            "range": "± 82466",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22355101,
            "range": "± 482107",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2800335,
            "range": "± 96994",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 42861474,
            "range": "± 1388090",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2166696,
            "range": "± 14714",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 24365081,
            "range": "± 234415",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2254745,
            "range": "± 25309",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 23118076,
            "range": "± 206555",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 1403801,
            "range": "± 26720",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 813286,
            "range": "± 3595",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 798250,
            "range": "± 7567",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1496512,
            "range": "± 20916",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 21345254,
            "range": "± 454136",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1433052,
            "range": "± 9237",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 26690281,
            "range": "± 484680",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1388800,
            "range": "± 24288",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 14612694,
            "range": "± 346786",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 812634,
            "range": "± 4292",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8172358,
            "range": "± 119046",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 868743,
            "range": "± 3630",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4414678,
            "range": "± 83506",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 492488,
            "range": "± 1578",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2466327,
            "range": "± 6024",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 595363,
            "range": "± 2308",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 12410888,
            "range": "± 539552",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1771599,
            "range": "± 20292",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 26625734,
            "range": "± 1555233",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1165872,
            "range": "± 19442",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 27391216,
            "range": "± 1498973",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 605464,
            "range": "± 6382",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 11369403,
            "range": "± 297556",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 895258,
            "range": "± 4925",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9333713,
            "range": "± 359245",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1671310,
            "range": "± 8622",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 26594736,
            "range": "± 971415",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 568,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 676,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1444,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1121,
            "range": "± 3",
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
          "id": "a942f192122ef78154b78c0b5f0cf2639c4ab917",
          "message": "Merge pull request #258 from syangkkim/claude/next-issue-mOzBd",
          "timestamp": "2026-03-27T19:47:51+09:00",
          "tree_id": "4cae062f72b7855aa530f800d799efc9c5c51fe7",
          "url": "https://github.com/syangkkim/dkit/commit/a942f192122ef78154b78c0b5f0cf2639c4ab917"
        },
        "date": 1774609010842,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2169754,
            "range": "± 4812",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22757479,
            "range": "± 341836",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2765249,
            "range": "± 150822",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 39689887,
            "range": "± 369178",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2202838,
            "range": "± 14056",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 23993291,
            "range": "± 685512",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2201175,
            "range": "± 17538",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22150043,
            "range": "± 123792",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 1368561,
            "range": "± 11073",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 793243,
            "range": "± 1776",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 811329,
            "range": "± 8487",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1352053,
            "range": "± 12851",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 18285302,
            "range": "± 388341",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1484218,
            "range": "± 21575",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 25435830,
            "range": "± 471635",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1453659,
            "range": "± 20667",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 15019214,
            "range": "± 147015",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 818460,
            "range": "± 5911",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8220816,
            "range": "± 67729",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 879664,
            "range": "± 7565",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4353509,
            "range": "± 27145",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 493414,
            "range": "± 2338",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2477461,
            "range": "± 15922",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 600065,
            "range": "± 3221",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 12438463,
            "range": "± 504203",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1798908,
            "range": "± 38889",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 27516382,
            "range": "± 1626659",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1204193,
            "range": "± 8652",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 29897985,
            "range": "± 1935290",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 601722,
            "range": "± 3869",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 12232214,
            "range": "± 368671",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 896031,
            "range": "± 13098",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9935389,
            "range": "± 554479",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1718441,
            "range": "± 9004",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 26733276,
            "range": "± 1000105",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 577,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 704,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1535,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1155,
            "range": "± 7",
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
          "id": "4d5f8d96cba2e1a1231ecd05a3e7ea0f3af9abdc",
          "message": "Merge pull request #259 from syangkkim/claude/next-issue-0boxR",
          "timestamp": "2026-03-27T20:01:07+09:00",
          "tree_id": "a1b11fe79e41360379d3aa0d45d365988672b9e9",
          "url": "https://github.com/syangkkim/dkit/commit/4d5f8d96cba2e1a1231ecd05a3e7ea0f3af9abdc"
        },
        "date": 1774609805316,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2188429,
            "range": "± 13872",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22429649,
            "range": "± 600774",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2791438,
            "range": "± 57684",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 39392961,
            "range": "± 520776",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2159181,
            "range": "± 47313",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21896769,
            "range": "± 928082",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2234019,
            "range": "± 11265",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22325583,
            "range": "± 125692",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 1363182,
            "range": "± 26036",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 812909,
            "range": "± 10779",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 792341,
            "range": "± 3838",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1341663,
            "range": "± 56639",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17512415,
            "range": "± 268928",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1461859,
            "range": "± 21897",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 24031701,
            "range": "± 438785",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1366345,
            "range": "± 24593",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13735323,
            "range": "± 139029",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 827844,
            "range": "± 2875",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8218262,
            "range": "± 420360",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 864992,
            "range": "± 14860",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4328781,
            "range": "± 10281",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 496149,
            "range": "± 3470",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2486872,
            "range": "± 166978",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 594130,
            "range": "± 11870",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 11183344,
            "range": "± 431916",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1801682,
            "range": "± 18943",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 24749291,
            "range": "± 1783048",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1179893,
            "range": "± 14923",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 24882755,
            "range": "± 2335011",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 601672,
            "range": "± 3540",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10246945,
            "range": "± 491448",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 883666,
            "range": "± 16004",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9174304,
            "range": "± 267176",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1679577,
            "range": "± 66818",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 24316212,
            "range": "± 1812833",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 563,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 699,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1503,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1130,
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
          "id": "5f309cc6bdd1ac05442b0ba09f9991dda7b58d24",
          "message": "Merge pull request #260 from syangkkim/claude/next-issue-KNCcM\n\nfeat: add --time execution profiling and --explain query plan",
          "timestamp": "2026-03-27T20:19:09+09:00",
          "tree_id": "79cd9ab1c501818fe6beecbcbe57a9f5baffcf1c",
          "url": "https://github.com/syangkkim/dkit/commit/5f309cc6bdd1ac05442b0ba09f9991dda7b58d24"
        },
        "date": 1774610886440,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2190613,
            "range": "± 15513",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22100961,
            "range": "± 425026",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2775709,
            "range": "± 89873",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 37743017,
            "range": "± 412780",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2173315,
            "range": "± 39521",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21921418,
            "range": "± 210067",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2212019,
            "range": "± 16295",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22062918,
            "range": "± 120691",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 1364767,
            "range": "± 33028",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 791066,
            "range": "± 3451",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 810311,
            "range": "± 2571",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1359924,
            "range": "± 12125",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 17481733,
            "range": "± 186356",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1435018,
            "range": "± 26624",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 23880451,
            "range": "± 563818",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1383671,
            "range": "± 11384",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13766263,
            "range": "± 119968",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 806854,
            "range": "± 2613",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8078320,
            "range": "± 81806",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 879015,
            "range": "± 4073",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4399269,
            "range": "± 26223",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 500141,
            "range": "± 1782",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2513802,
            "range": "± 10339",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 594975,
            "range": "± 6172",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 11134121,
            "range": "± 145671",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1812741,
            "range": "± 26800",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 23803168,
            "range": "± 984852",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1179512,
            "range": "± 15112",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 24668415,
            "range": "± 741167",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 599238,
            "range": "± 1857",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10484156,
            "range": "± 219750",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 892412,
            "range": "± 2718",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9185895,
            "range": "± 167674",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1673025,
            "range": "± 36748",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 23139365,
            "range": "± 453632",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 561,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 699,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1534,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1144,
            "range": "± 6",
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
          "id": "efd97ede4df3566a00e22bf8487fb6a3251a7b8e",
          "message": "Merge pull request #261 from syangkkim/claude/next-issue-0fVue",
          "timestamp": "2026-03-27T20:39:20+09:00",
          "tree_id": "4bf7f08a9121d368938c0be9f6f550cd77f2a501",
          "url": "https://github.com/syangkkim/dkit/commit/efd97ede4df3566a00e22bf8487fb6a3251a7b8e"
        },
        "date": 1774612093182,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 2202747,
            "range": "± 18516",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 22521458,
            "range": "± 361739",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2889576,
            "range": "± 160734",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 41568277,
            "range": "± 1867379",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 2205558,
            "range": "± 21340",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 21707280,
            "range": "± 268344",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 2275282,
            "range": "± 17612",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 22450616,
            "range": "± 182567",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 1415619,
            "range": "± 21285",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 821301,
            "range": "± 16526",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 844643,
            "range": "± 19145",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1394117,
            "range": "± 33568",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 18437398,
            "range": "± 156819",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1491017,
            "range": "± 28158",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 25709362,
            "range": "± 763085",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1395080,
            "range": "± 17271",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 13796307,
            "range": "± 136851",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 852436,
            "range": "± 8023",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 8250929,
            "range": "± 74229",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 894350,
            "range": "± 5556",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 4448578,
            "range": "± 34013",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 496510,
            "range": "± 4159",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2501784,
            "range": "± 18998",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 611042,
            "range": "± 4997",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 12350907,
            "range": "± 250445",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 2008281,
            "range": "± 24795",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 24860511,
            "range": "± 818485",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1196508,
            "range": "± 15225",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 26418660,
            "range": "± 1279581",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 616748,
            "range": "± 7806",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 11251433,
            "range": "± 202003",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 896874,
            "range": "± 9853",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 9107865,
            "range": "± 99015",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1710153,
            "range": "± 31529",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 23968516,
            "range": "± 559005",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 573,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 703,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1525,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 1178,
            "range": "± 21",
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
          "id": "2c6fe11942a197b22ecf0961d1d11b0d4f23dc57",
          "message": "Merge pull request #262 from syangkkim/claude/next-issue-kJZBE",
          "timestamp": "2026-03-27T20:57:37+09:00",
          "tree_id": "99811a762f1624c6d081e28ed9c00b099275664c",
          "url": "https://github.com/syangkkim/dkit/commit/2c6fe11942a197b22ecf0961d1d11b0d4f23dc57"
        },
        "date": 1774613194096,
        "tool": "cargo",
        "benches": [
          {
            "name": "convert_json_to_csv/1000",
            "value": 1791522,
            "range": "± 54686",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_csv/10000",
            "value": 21000585,
            "range": "± 454225",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/1000",
            "value": 2586404,
            "range": "± 5566",
            "unit": "ns/iter"
          },
          {
            "name": "convert_csv_to_json/10000",
            "value": 37241979,
            "range": "± 583857",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/1000",
            "value": 1785823,
            "range": "± 12527",
            "unit": "ns/iter"
          },
          {
            "name": "convert_json_to_jsonl/10000",
            "value": 19791496,
            "range": "± 296079",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/1000",
            "value": 1960256,
            "range": "± 88035",
            "unit": "ns/iter"
          },
          {
            "name": "convert_jsonl_to_csv/10000",
            "value": 20745059,
            "range": "± 442296",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/json",
            "value": 1295806,
            "range": "± 11170",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/csv",
            "value": 739483,
            "range": "± 1421",
            "unit": "ns/iter"
          },
          {
            "name": "value_serialize/jsonl",
            "value": 666182,
            "range": "± 1569",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/1000",
            "value": 1101379,
            "range": "± 7327",
            "unit": "ns/iter"
          },
          {
            "name": "json_read/10000",
            "value": 15573621,
            "range": "± 272744",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/1000",
            "value": 1343609,
            "range": "± 20104",
            "unit": "ns/iter"
          },
          {
            "name": "json_write/10000",
            "value": 22138514,
            "range": "± 383611",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/1000",
            "value": 1220461,
            "range": "± 84368",
            "unit": "ns/iter"
          },
          {
            "name": "csv_read/10000",
            "value": 12952741,
            "range": "± 360564",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/1000",
            "value": 733169,
            "range": "± 3732",
            "unit": "ns/iter"
          },
          {
            "name": "csv_write/10000",
            "value": 7485542,
            "range": "± 87282",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/100",
            "value": 741393,
            "range": "± 9492",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_read/500",
            "value": 3718777,
            "range": "± 7829",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/100",
            "value": 441510,
            "range": "± 610",
            "unit": "ns/iter"
          },
          {
            "name": "yaml_write/500",
            "value": 2217235,
            "range": "± 7909",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/1000",
            "value": 465540,
            "range": "± 1035",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter/10000",
            "value": 10792079,
            "range": "± 287884",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/1000",
            "value": 1653953,
            "range": "± 11582",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort/10000",
            "value": 25934417,
            "range": "± 843361",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/1000",
            "value": 1125646,
            "range": "± 10297",
            "unit": "ns/iter"
          },
          {
            "name": "query_sort_desc/10000",
            "value": 27696122,
            "range": "± 855540",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/1000",
            "value": 606531,
            "range": "± 912",
            "unit": "ns/iter"
          },
          {
            "name": "query_sum/10000",
            "value": 10145175,
            "range": "± 431739",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/1000",
            "value": 832991,
            "range": "± 1265",
            "unit": "ns/iter"
          },
          {
            "name": "query_group_by/10000",
            "value": 10010249,
            "range": "± 222553",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/1000",
            "value": 1533817,
            "range": "± 9637",
            "unit": "ns/iter"
          },
          {
            "name": "query_filter_sort/10000",
            "value": 24141617,
            "range": "± 715893",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where age > 30",
            "value": 518,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | sort score desc | limit 100",
            "value": 577,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | group_by category | select category, count",
            "value": 1279,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "query_parse/. | where active == true | sort age | limit 50",
            "value": 953,
            "range": "± 5",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}