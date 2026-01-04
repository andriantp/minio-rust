# S3 Rust + MinIO – Complete Implementation

This project is a full MinIO (S3‑compatible) implementation using the **Rust AWS SDK**, designed with a clean, modular, and scalable architecture:

* **RepositoryBuilder** → builds the client + ensures bucket availability
* **Repository** → façade for accessing services
* **BucketService** → bucket operations (list, create, ensure, delete, stats)
* **ObjectService** → object operations (upload, download, delete, info, checksum)

---
## 🔗 Reference

Full article:
[Implementation MinIO (S3) with Rust](https://andriantriputra.medium.com/be-rust-implementation-minio-s3-with-rust-fb1c7300aa01)

---
## 🚀 MinIO Setup via Docker

Start MinIO using docker compose:

```bash
make up
```

MinIO runs on:

* **Console** → [http://localhost:9001](http://localhost:9001)
* **API** → [http://localhost:9000](http://localhost:9000)

Default credentials:

```
username: minio
password: mini@123
```

---

## 🔧 Environment (.env)

Create a `.env` file in the root folder:

```
AWS_ACCESS_KEY_ID=minio
AWS_SECRET_ACCESS_KEY=mini@123
AWS_REGION=us-east-1
MINIO_ENDPOINT=http://localhost:9000
MINIO_BUCKET=rust-bucket
```

---

## 🦀 Running the Rust Application

### Build

```bash
cd s3-rust
cargo build
```

### CLI

```bash
cargo run -- bucket-list
```

---

## 🧱 Architecture Overview

### 1. RepositoryBuilder

* Builds AWS config
* Builds S3 client
* Ensures bucket exists
* Produces a fully valid **Repository**

### 2. Repository

A façade layer that exposes service accessors:

```rust
repo.bucket().list()
repo.object().upload(...)
```

### 3. Bucket-Service

* list
* create
* ensure
* delete
* delete_objects
* stats

### 4. Object-Service

* upload (with checksum metadata)
* download
* list
* delete
* info (stat object)

### 5. Utils

* sha256_file
* sha256_bytes


---

## 🎯 Goals & Benefits

* Clean architecture (builder → repo → service)
* Modular & scalable
* Easy to extend (multipart upload, presigned URLs, etc.)
* Ideal for Medium tutorials
* Production-ready (checksum, logging, error handling)

---

## 📌 Future Enhancements

* Object verification CLI
* Presigned upload/download
* Multipart upload
* Delete by prefix
* Object listing pagination


---

## Author

Andrian Tri Putra
- [Medium](https://andriantriputra.medium.com/)
- [andriantp](https://github.com/andriantp)
- [AndrianTriPutra](https://github.com/AndrianTriPutra)

---

## 📝 License
Licensed under the Apache License 2.0
