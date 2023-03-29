#ifdef __cplusplus
extern "C" {
#endif

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>


typedef enum SafeUriPackageOrWrapperType {
  SUPW_Uri,
  SUPW_WasmWrapper,
  SUPW_PluginWrapper,
  SUPW_WasmPackage,
  SUPW_PluginPackage,
} SafeUriPackageOrWrapperType;

typedef enum SafeUriResolverLikeType {
  SURL_Resolver,
  SURL_Redirect,
  SURL_WasmPackage,
  SURL_PluginPackage,
  SURL_WasmWrapper,
  SURL_PluginWrapper,
} SafeUriResolverLikeType;

typedef struct PluginPtrHandle PluginPtrHandle;

typedef const int8_t * _Nonnull (*PluginInvokeFn)(const void * _Nonnull plugin_ptr, const int8_t * _Nonnull method_name, const int8_t * _Nonnull params, void * _Nonnull invoker);

typedef struct SafeUriResolverLikeVariant {
  enum SafeUriResolverLikeType _type;
  void *data;
  const char *uri;
} SafeUriResolverLikeVariant;

typedef struct ExtPluginModule {
  void *env;
  struct PluginPtrHandle *ptr_handle;
  PluginInvokeFn plugin_invoke;
} ExtPluginModule;

typedef struct SafeUriPackageOrWrapper {
  const char *uri;
  enum SafeUriPackageOrWrapperType data_type;
  void *data;
} SafeUriPackageOrWrapper;

typedef struct Buffer {
  uint8_t *data;
  uintptr_t len;
} Buffer;

void *new_builder_config(void);

void add_env(void *builder_config_ptr, const char *uri, const char *env);

void remove_env(void *builder_config_ptr, const char *uri);

void set_env(void *builder_config_ptr, const char *uri, const char *env);

void add_interface_implementation(void *builder_config_ptr,
                                  const char *interface_uri,
                                  const char *implementation_uri);

void remove_interface_implementation(void *builder_config_ptr,
                                     const char *interface_uri,
                                     const char *implementation_uri);

void add_wasm_wrapper(void *builder_config_ptr, const char *uri, void *wrapper);

void add_plugin_wrapper(void * _Nonnull builder_config_ptr,
                        const char * _Nonnull uri,
                        void * _Nonnull plugin_ptr,
                        _Nonnull PluginInvokeFn plugin_invoke_fn);

void remove_wrapper(void *builder_config_ptr, const char *uri);

void add_wasm_package(void *builder_config_ptr, const char *uri, void *package);

void add_plugin_package(void *builder_config_ptr, const char *uri, void *package);

void remove_package(void *builder_config_ptr, const char *uri);

void add_redirect(void *builder_config_ptr, const char *from, const char *to);

void remove_redirect(void *builder_config_ptr, const char *from);

void add_wrapper_resolver(void *builder_config_ptr,
                          struct SafeUriResolverLikeVariant resolver);

void add_redirect_resolver(void *builder_config_ptr,
                           struct SafeUriResolverLikeVariant resolver);

void add_package_resolver(void *builder_config_ptr,
                          struct SafeUriResolverLikeVariant resolver);

void add_resolver(void *builder_config_ptr, struct SafeUriResolverLikeVariant resolver);

void set_plugin_env(struct ExtPluginModule *plugin_ptr, const char *env_json_str);

const int8_t *get_plugin_env(struct ExtPluginModule *plugin_ptr, const char *key);

void *create_static_resolver(const struct SafeUriPackageOrWrapper *entries,
                             uintptr_t len);

void *create_extendable_resolver(void);

void *create_client(void *builder_config_ptr);

const char * _Nonnull invoke_raw(void *client_ptr,
                       const char *uri,
                       const char *method,
                       const char *args,
                       const char *env);

const struct Buffer *encode(const char *json_str);

#ifdef __cplusplus
}
#endif
