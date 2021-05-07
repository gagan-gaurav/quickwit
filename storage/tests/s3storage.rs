/*
    Quickwit
    Copyright (C) 2021 Quickwit Inc.

    Quickwit is offered under the AGPL v3.0 and as commercial software.
    For commercial licensing, contact us at hello@quickwit.io.

    AGPL:
    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as
    published by the Free Software Foundation, either version 3 of the
    License, or (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::path::Path;

use quickwit_storage::{PutPayload, S3CompatibleObjectStorage, S3MultiPartPolicy, Storage};
use rusoto_core::Region;

#[tokio::test]
async fn test_upload_single_part_file() -> anyhow::Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    let object_storage =
        S3CompatibleObjectStorage::new(Region::UsEast1, "quickwit-integration-test")?;
    object_storage
        .put(
            Path::new("test-s3-compatible-storage/hello_small.txt"),
            PutPayload::from(b"hello, happy tax payer!".to_vec()),
        )
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_upload_multiple_part_file() -> anyhow::Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    let mut object_storage =
        S3CompatibleObjectStorage::new(Region::UsEast1, "quickwit-integration-test")?;
    object_storage.set_policy(S3MultiPartPolicy {
        target_part_num_bytes: 5 * 1_024 * 1_024, //< the minimum on S3 is 5MB.
        max_num_parts: 10_000,
        multipart_threshold_num_bytes: 10_000_000,
        max_object_num_bytes: 5_000_000_000_000,
        max_concurrent_upload: 100,
    });
    let test_buffer = vec![0u8; 15_000_000];
    object_storage
        .put(
            Path::new("test-s3-compatible-storage/hello_large.txt"),
            PutPayload::from(test_buffer),
        )
        .await?;
    Ok(())
}

#[cfg(feature = "testsuite")]
#[tokio::test]
async fn test_suite() -> anyhow::Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    let mut object_storage =
        S3CompatibleObjectStorage::new(s3_client(), "quickwit-integration-test");
    quickwit_storage::tests::storage_test_suite(&mut object_storage).await?;
    Ok(())
}
