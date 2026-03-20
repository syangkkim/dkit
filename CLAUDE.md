# CLAUDE.md - dkit Development Guide

## Project Overview

dkit (Data Kit) — 모든 데이터 포맷을 하나의 CLI로 변환하고 쿼리하는 스위스 아미 나이프.
JSON, CSV, YAML, TOML 간 양방향 변환 + 통합 쿼리 + 테이블 출력.

- Language: Rust
- License: MIT

## Development Workflow

### Task Management

모든 기획, 계획, 진행, 상태 관리는 **GitHub Issues와 Project 보드**에서 관리한다.
저장소 내부 문서로 태스크를 관리하지 않는다.

- **Project Board**: https://github.com/users/syangkkim/projects/2
- **Issues**: https://github.com/syangkkim/dkit/issues

### How to Work on Issues

개발자가 "N번 이슈를 개발해줘"라고 요청하면:

1. `gh issue view N`으로 이슈 내용 확인
2. 이슈의 "의존 이슈"가 있으면 해당 이슈들이 완료되었는지 확인
3. `docs/` 폴더의 관련 문서를 참조하여 구현 세부사항 파악
4. 구현 완료 후 커밋하고 브랜치에 푸시 (PR 생성은 하지 않는다 — 사용자가 직접 UI에서 생성)

### Reference Documents (docs/)

버전 관리가 필요한 기술 문서만 `docs/`에서 관리한다:

- `docs/architecture.md` — 프로젝트 구조, 모듈 구조, 의존성
- `docs/technical-spec.md` — Value 타입, 트레이트, 에러 타입, 쿼리 엔진 설계
- `docs/cli-spec.md` — 서브커맨드별 CLI 인터페이스 명세 (옵션, 사용법)
- `docs/query-syntax.md` — 쿼리 문법 명세 (EBNF, 연산자, 예제)

### Labels

- `v0.1.0` ~ `v0.4.0`: 버전 마일스톤
- `core`: 핵심 기능 (Value, 트레이트, 에러)
- `format`: 데이터 포맷 관련 (JSON, CSV, YAML, TOML)
- `cli`: CLI 인터페이스 및 서브커맨드
- `query`: 쿼리 엔진
- `testing`: 테스트 관련
- `infra`: 인프라 및 CI/CD
- `release`: 릴리스 관련

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
