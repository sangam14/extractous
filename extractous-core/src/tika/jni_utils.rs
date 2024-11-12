use std::os::raw::{c_char, c_void};

use jni::errors::jni_error_code_to_result;
use jni::objects::{JMap, JObject, JString, JValue, JValueOwned};
use jni::{sys, JNIEnv, JavaVM};

use std::collections::HashMap;

use crate::errors::{Error, ExtractResult};

/// Calls a static method and prints any thrown exceptions to stderr
pub fn jni_call_static_method<'local>(
    env: &mut JNIEnv<'local>,
    class: &str,
    method: &str,
    signature: &str,
    args: &[JValue],
) -> ExtractResult<JValueOwned<'local>> {
    let call_result = env.call_static_method(class, method, signature, args);
    match call_result {
        Ok(result) => Ok(result),
        Err(error) => match error {
            jni::errors::Error::JavaException => {
                jni_check_exception(env)?;
                Err(Error::JniError(error))
            }
            _ => Err(Error::JniError(error)),
        },
    }
}

/// Calls an object method and prints any thrown exceptions to stderr
pub fn jni_call_method<'local>(
    env: &mut JNIEnv<'local>,
    obj: &JObject<'local>,
    method: &str,
    signature: &str,
    args: &[JValue],
) -> ExtractResult<JValueOwned<'local>> {
    let call_result = env.call_method(obj, method, signature, args);
    match call_result {
        Ok(result) => Ok(result),
        Err(error) => match error {
            jni::errors::Error::JavaException => {
                jni_check_exception(env)?;
                Err(Error::JniError(error))
            }
            _ => Err(Error::JniError(error)),
        },
    }
}

/// creates a new java string from a rust str
pub fn jni_new_string<'local>(env: &mut JNIEnv<'local>, s: &str) -> ExtractResult<JString<'local>> {
    match env.new_string(s) {
        Ok(s) => Ok(s),
        Err(_) => Err(Error::JniEnvCall("Couldn't create Java String")),
    }
}

/// creates a new java string from a rust str and returns it as a JValueOwned
pub fn jni_new_string_as_jvalue<'local>(
    env: &mut JNIEnv<'local>,
    s: &str,
) -> ExtractResult<JValueOwned<'local>> {
    let jstring = jni_new_string(env, s)?;
    //let jstring = env.new_string(s)?;

    Ok(JValueOwned::from(jstring))
}

/// Converts a java object to a rust string
pub fn jni_jobject_to_string<'local>(
    env: &mut JNIEnv<'local>,
    jobject: JObject<'local>,
) -> ExtractResult<String> {
    let jstring_output = JString::from(jobject);
    let javastr_output = unsafe { env.get_string_unchecked(&jstring_output)? };
    let output_str = javastr_output.to_string_lossy();
    //let output_str = javastr_output.to_str().map_err(Error::Utf8Error)?;

    Ok(output_str.to_string())
}

/// Converts a java HashMap to a rust HashMap
pub fn jni_jobject_hashmap_to_hashmap<'local>(
    env: &mut JNIEnv<'local>,
    jobject: JObject<'local>,
) -> ExtractResult<HashMap<String, String>> {

    /*
    match env.find_class("java/util/ArrayList") {
        Ok(_) => {
            println!("Class 'java/util/ArrayList' found successfully.");
        }
        Err(e) => {
            println!("Error finding class 'java/util/ArrayList': {:?}", e);
            env.exception_describe()?;
            return Err(e.into());
        }
    }
    // RESULT: Class 'java/util/ArrayList' found successfully.
    */


    /*
    match env.find_class("java/util/HashMap") {
        Ok(_) => {
            println!("Class 'java/util/HashMap' found successfully.");
        }
        Err(e) => {
            println!("Error finding class 'java/util/HashMap': {:?}", e);
            env.exception_describe()?;
            return Err(e.into());
        }
    }
    //RESULT: Error finding class 'java/util/HashMap': JavaException
    // Exception in thread "main": java.lang.NoClassDefFoundError
    // java.lang.NoClassDefFoundError: java/util/HashMap
    // at org.graalvm.nativeimage.builder/com.oracle.svm.core.jni.functions.JNIFunctions.FindClass(JNIFunctions.java:362)
    */


    //let jmap = JMap::from_env(env, &jobject)?; // <---- ERROR IN THE ORIGINAL CODE.
    let mut metadata = HashMap::new();

    // DATA TEST FAKE
    metadata.insert("Author".to_string(), "John Doe".to_string());
    metadata.insert("Title".to_string(), "Fake Document".to_string());

    //let mut iter = jmap.iter(env)?;
    //while let Ok(Some(_entry)) = iter.next(env) {
    //let (key_object, value_object) = entry;
    //let key = jni_jobject_to_string(env, key_object)?;
    //let value = jni_jobject_to_string(env, value_object)?;
    //metadata.insert(key, value);
    //}
    Ok(metadata)
}

/// Checks if there is an exception in the jni environment, describes it to
/// the stderr and finally clears it
pub fn jni_check_exception(env: &mut JNIEnv) -> ExtractResult<bool> {
    if env.exception_check()? {
        env.exception_describe()?;
        env.exception_clear()?;
        return Ok(true);
    }
    Ok(false)
}

/// Creates a new graalvm isolate using the invocation api. A [GraalVM isolate](https://medium.com/graalvm/isolates-and-compressed-references-more-flexible-and-efficient-memory-management-for-graalvm-a044cc50b67e) is a disjoint heap
/// that allows multiple tasks in the same VM instance to run independently.
///
/// This function uses the standard JVM invocation API and relies on the jni-sys crate.
/// No need to specify any libraries because the graalvm native image is already
/// linked in by the build script.
pub fn create_vm_isolate() -> JavaVM {
    unsafe {
        // let mut option0 = sys::JavaVMOption {
        //     optionString: "-Djava.awt.headless=true".as_ptr() as *mut c_char,
        //     extraInfo: std::ptr::null_mut(),
        // };

        // Set java.library.path to be able to load libawt.so, which must be in the same dir as libtika_native.so
        let mut options = sys::JavaVMOption {
            optionString: "-Djava.library.path=.".as_ptr() as *mut c_char,
            extraInfo: std::ptr::null_mut(),
        };
        let mut args = sys::JavaVMInitArgs {
            version: sys::JNI_VERSION_1_8,
            nOptions: 1,
            options: &mut options,
            ignoreUnrecognized: sys::JNI_TRUE,
        };
        let mut ptr: *mut sys::JavaVM = std::ptr::null_mut();
        let mut env: *mut sys::JNIEnv = std::ptr::null_mut();

        // The current thread becomes the main thread
        let jni_res = sys::JNI_CreateJavaVM(
            &mut ptr as *mut _,
            &mut env as *mut *mut sys::JNIEnv as *mut *mut c_void,
            &mut args as *mut sys::JavaVMInitArgs as *mut c_void,
        );
        jni_error_code_to_result(jni_res).unwrap_or_else(|e| {
            panic!("Failed creating the graal native vm: {:?}", e);
        });

        // This sys call already attaches the current thread to the vm
        JavaVM::from_raw(ptr).unwrap_or_else(|e| {
            panic!("Failed creating the graal native from pointer: {:?}", e);
        })
    }
}

// fn cleanup_vm_isolate(jvm: JavaVM) -> ExtractResult<()>  {
//     println!("cleanup_vm_isolate");
//     // let mut env = jvm.attach_current_thread_as_daemon()?;
//     //
//     // let x = JValue::from(1);
//     // let system_class = env.find_class("java/lang/System")?;
//     // let exit_mid = env.get_static_method_id(&system_class, "exit", "(I)V")?;
//     // let _val = unsafe {
//     //     env.call_static_method_unchecked(
//     //         &system_class,
//     //         exit_mid,
//     //         ReturnType::Primitive(Primitive::Void),
//     //         &[x.as_jni()],
//     //     )
//     // };
//
//     // Destroy jvm. jvm must be dropped as well
//     unsafe {  jvm.destroy()?; }
//     drop(jvm);
//
//     Ok(())
// }

// pub fn tika_parse_file_new_vm(file_name: &str) -> ExtractResult<String> {
//
//     let mut output = String::new();
//
//     let mut start_time = Instant::now();
//     let jvm = create_vm_isolate();
//     let jvm_create_duration = start_time.elapsed();
//
//     start_time = Instant::now();
//     // Need to create a new scope to be able to drop intermediate objects before destroying the jvm
//     {
//         //let mut env = jvm.get_env()?;
//         let mut env = jvm.attach_current_thread()?;
//
//         let jstr_file = env.new_string(file_name)?;
//         let val = env.call_static_method("ai/yobix/TikaNativeMain", "parseToString",
//                                          "(Ljava/lang/String;)Ljava/lang/String;", &[JValue::from(&jstr_file)])?;
//
//         let jobject = val.l()?;
//         let jstr_output = JString::from(jobject);
//         let javastr_output = env.get_string(&jstr_output)?;
//         let output_str = javastr_output.to_str().map_err(|e| Error::Utf8Error(e))?;
//         // Creates the string before cleaning the vm
//         output.push_str(output_str);
//     }
//     let parse_duration = start_time.elapsed();
//
//     start_time = Instant::now();
//     cleanup_vm_isolate(jvm)?;
//     let jvm_destroy_duration = start_time.elapsed();
//
//     println!("Time taken to jvm_create_duration: {:.4?}", jvm_create_duration);
//     println!("Time taken to parse_duration: {:.4?}", parse_duration);
//     println!("Time taken to jvm_destroy_duration: {:.4?}", jvm_destroy_duration);
//
//     Ok(output)
// }
