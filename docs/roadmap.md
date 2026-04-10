# dkit Roadmap

> v0.4.0 ~ v1.0.0 로드맵. 세부 이슈는 [GitHub Issues](https://github.com/syang0531/dkit/issues)에서 관리.

## 버전 요약

| 버전 | 테마 | 핵심 기능 |
|------|------|-----------|
| v0.4.0 | 텍스트 데이터 포맷 확장 | XML, JSONL, 포맷 자동 감지 |
| v0.5.0 | 출력 포맷 강화 | Markdown/HTML 테이블, 테이블 커스터마이징, 인코딩 |
| v0.6.0 | 외부 데이터 소스 | Excel, SQLite, stdin/stdout 파이프라인, 일괄 변환 |
| v0.7.0 | 데이터 분석 진입 | Parquet, 집계 함수, GROUP BY, 스트리밍 처리 |
| v0.8.0 | 데이터 검증/비교 | diff 고도화, validate, stats 확장, sample, flatten |
| v0.9.0 | 개발자 경험(DX) | 설정 파일, 쉘 자동완성, --watch, 별칭, 벤치마크 |
| v1.0.0 | 안정화 + API 확정 | 라이브러리 분리, 크로스 플랫폼 배포, 종합 문서 |

---

## v0.4.0 — 텍스트 데이터 포맷 확장

- XML Reader/Writer (속성, 네임스페이스, CDATA 처리)
- JSONL (JSON Lines) Reader/Writer
- 포맷 자동 감지 개선 (확장자 + 콘텐츠 스니핑)
- convert 서브커맨드에 새 포맷 통합
- 통합 테스트 및 문서 업데이트

## v0.5.0 — 출력 포맷 강화 + 인코딩

- Markdown 테이블 출력 (GFM 형식)
- HTML 테이블 출력 (인라인 스타일 옵션)
- 테이블 출력(view) 커스터마이징 (너비, 테두리, 색상, 행 번호)
- 출력 포맷 선택 통합 (`--format` 옵션 표준화)
- 인코딩 지원 (UTF-8 외: EUC-KR, Shift-JIS 등)

## v0.6.0 — 외부 데이터 소스 읽기

- Excel (.xlsx) Reader (시트 선택, 셀 타입 변환)
- SQLite Reader (테이블/커스텀 쿼리)
- stdin/stdout 파이프라인 스트리밍 지원
- 다중 파일 일괄 변환 (batch convert)
- 데이터 정렬/필터링 옵션 (`--sort-by`, `--head`, `--tail`)

## v0.7.0 — 데이터 분석 영역 진입

- Parquet Reader/Writer (Arrow 생태계)
- 쿼리 집계 함수 (count, sum, avg, min, max, distinct)
- GROUP BY 절 + HAVING
- 대용량 파일 스트리밍 처리 (청크 기반)
- 쿼리 함수 확장 (문자열, 날짜, 수학, 타입 변환)

## v0.8.0 — 데이터 검증 및 비교

- diff 서브커맨드 고도화 (비교 모드, 출력 포맷, 배열 비교 전략)
- validate 서브커맨드 (JSON Schema 기반 검증)
- stats 서브커맨드 확장 (필드별 상세 통계, 히스토그램)
- sample 서브커맨드 (무작위/등간격/층화 샘플링)
- flatten/unflatten 서브커맨드 (중첩 구조 평탄화)

## v0.9.0 — 개발자 경험(DX) 완성

- 설정 파일 (`~/.dkit.toml`, 프로젝트별 `.dkit.toml`)
- 쉘 자동완성 (bash, zsh, fish, PowerShell)
- `--watch` 모드 (파일 변경 감지 자동 실행)
- 에러 메시지 개선 (줄/열 번호, 수정 제안, miette)
- 별칭(alias) 시스템
- 성능 벤치마크 및 최적화

## v1.0.0 — 안정화 + API 확정

- `dkit-core` 라이브러리 크레이트 분리
- 공개 API 확정 + Semantic Versioning 보장
- 크로스 플랫폼 바이너리 배포 (Linux/macOS/Windows)
- Feature Flags로 선택적 의존성 관리
- man page 생성 및 내장 도움말 개선
- 종합 문서 (README, 튜토리얼, 예제 모음, 마이그레이션 가이드)
- 안정화 테스트 및 QA (퍼징, 메모리 검사, 크로스 플랫폼)
