/*
* Copyright (c) 2025 sockentrocken
*
* Redistribution and use in source and binary forms, with or without
* modification, are permitted provided that the following conditions are met:
*
* 1. Redistributions of source code must retain the above copyright notice,
* this list of conditions and the following disclaimer.
*
* 2. Redistributions in binary form must reproduce the above copyright notice,
* this list of conditions and the following disclaimer in the documentation
* and/or other materials provided with the distribution.
*
* Subject to the terms and conditions of this license, each copyright holder
* and contributor hereby grants to those receiving rights under this license
* a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable
* (except for failure to satisfy the conditions of this license) patent license
* to make, have made, use, offer to sell, sell, import, and otherwise transfer
* this software, where such license applies only to those patent claims, already
* acquired or hereafter acquired, licensable by such copyright holder or
* contributor that are necessarily infringed by:
*
* (a) their Contribution(s) (the licensed copyrights of copyright holders and
* non-copyrightable additions of contributors, in source or binary form) alone;
* or
*
* (b) combination of their Contribution(s) with the work of authorship to which
* such Contribution(s) was added by such copyright holder or contributor, if,
* at the time the Contribution is added, such addition causes such combination
* to be necessarily infringed. The patent license shall not apply to any other
* combinations which include the Contribution.
*
* Except as expressly stated above, no rights or licenses from any copyright
* holder or contributor is granted under this license, whether expressly, by
* implication, estoppel or otherwise.
*
* DISCLAIMER
*
* THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
* AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
* IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
* DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE
* FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
* DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
* SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
* CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
* OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
* OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

#[cfg(test)]
mod test_main {
    use crate::script::*;
    use crate::status::*;

    async fn test_folder(path: &str) {
        let path_list = std::fs::read_dir(format!("test/system/{path}")).unwrap();

        for entry in path_list {
            let entry = entry.unwrap().path().display().to_string();

            if let Err(error) = Script::new_test(&entry).await {
                println!("Assertion fail or panic in entry: \"{entry}\"");
                panic!("{error:?}");
            }
        }
    }

    #[tokio::test]
    async fn main() {
        let (_handle, _thread, _audio) = Status::window(&None).unwrap();

        test_folder("data").await;
        test_folder("file").await;
        test_folder("input").await;
        test_folder("lua").await;

        /*
        #[cfg(feature = "rapier3d")]
        test_folder("rapier3d").await;

        #[cfg(feature = "zip")]
        test_folder("zip").await;
        */

        #[cfg(feature = "request")]
        test_folder("request").await;

        /*
        // NOTE: you MUST have Steam running for this test.
        #[cfg(feature = "steam")]
        test_folder("steam").await;

        // NOTE: you MUST have Discord running for this test.
        #[cfg(feature = "discord")]
        test_folder("discord").await;

        #[cfg(feature = "video")]
        test_folder("video").await;
        */
    }
}
