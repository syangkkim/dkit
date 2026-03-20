# CLAUDE.md - dkit Development Guide

## Project Overview

dkit (Data Kit) — 모든 데이터 포맷을 하나의 CLI로 변환하고 쿼리하는 스위스 아미 나이프.
JSON, CSV, YAML, TOML 간 양방향 변환 + 통합 쿼리 + 테이블 출력.

- Language: Rust
- License: MIT

## Development Workflow

### Task Management

모든 기획, 계획, 진행, 상태 관리는 **GitHub Issues**에서 관리한다.
저장소 내부 문서로 태스크를 관리하지 않는다.

- **Issues**: https://github.com/syangkkim/dkit/issues

### How to Work on Issues

개발자가 "N번 이슈를 개발해줘"라고 요청하면:

1. `gh issue view N`으로 이슈 내용 확인
2. 이슈의 "의존 이슈"가 있으면 해당 이슈들이 완료되었는지 확인
3. `docs/` 폴더의 관련 문서를 참조하여 구현 세부사항 파악
4. 구현 및 테스트 완료 후 커밋
5. PR 생성 (`Closes #N`으로 이슈 자동 닫기 연결)

### PR 생성 규칙

- 이슈 작업이 완료되면 반드시 PR을 생성한다.
- PR 제목: 간결하게 변경 내용을 요약 (70자 이내)
- PR 본문에 `Closes #N`을 포함하여 이슈를 자동으로 닫는다.
- base 브랜치는 `main`으로 한다.

### Reference Documents (docs/)

버전 관리가 필요한 기술 문서만 `docs/`에서 관리한다:

- `docs/architecture.md` — 프로젝트 구조, 모듈 구조, 의존성
- `docs/technical-spec.md` — Value 타입, 트레이트, 에러 타입, 쿼리 엔진 설계
- `docs/cli-spec.md` — 서브커맨드별 CLI 인터페이스 명세 (옵션, 사용법)
- `docs/query-syntax.md` — 쿼리 문법 명세 (EBNF, 연산자, 예제)

### 이슈 레이블 체계

레이블은 접두사 기반으로 분류한다. 이슈 생성 시 `type:` + `area:` 조합을 기본으로 사용하고, 필요 시 `p0:`~`p2:` 우선순위를 추가한다.

| 카테고리 | 레이블 | 설명 |
|---------|--------|------|
| **타입** | `type:feature` | 새 기능 |
| | `type:bug` | 버그 수정 |
| | `type:refactor` | 리팩토링/개선 |
| | `type:docs` | 문서 작업 |
| | `type:infra` | CI/CD, 빌드 |
| **우선순위** | `p0:critical` | 즉시 해결 |
| | `p1:high` | 다음 작업 |
| | `p2:normal` | 일반 (기본값) |
| **컴포넌트** | `area:core` | 핵심 기능 (Value, 트레이트, 에러) |
| | `area:cli` | CLI 인터페이스 |
| | `area:format` | 데이터 포맷 (JSON, CSV, YAML, TOML) |
| | `area:query` | 쿼리 엔진 |

### 마일스톤 관리

버전 마일스톤은 GitHub Milestones 기능으로 관리한다 (레이블 아님).
이슈 생성 시 해당 마일스톤에 할당한다.

## Build & Test Commands

```bash
cargo build                    # 빌드
cargo test                     # 전체 테스트
cargo clippy -- -D warnings    # 린트
cargo fmt -- --check           # 포맷 검사
cargo run -- <subcommand>      # 실행
```

## Code Style

- `cargo fmt`으로 포맷팅
- `cargo clippy`으로 린트
- 에러 처리: `thiserror` (라이브러리 에러) + `anyhow` (애플리케이션 에러)
- 테스트: 각 모듈에 `#[cfg(test)]` 단위 테스트 + `tests/` 통합 테스트
