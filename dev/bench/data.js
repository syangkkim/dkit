window.BENCHMARK_DATA = {
  "lastUpdate": 1774405148613,
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
      }
    ]
  }
}