# # this is dumb
# # cargo install cargo-version-util
# # cargo install cargo-edit
# set current_version=cargo version-util get-version
# #cargo publish -p blf_lib-derivable
# echo %current_version%
import subprocess

print("blf_lib - publish script")
current_version = subprocess.check_output(['cargo', 'version-util', 'get-version']).decode().replace('\n', '').replace('\r', '')
print(f"current version = {current_version}")
print("please enter the new version")
new_version = input("new version = ")
subprocess.call(['cargo', 'version-util', 'set-version', new_version])

print("blf_lib-derivable: publishing")
try:
    subprocess.call(['cargo', 'publish', '-p', 'blf_lib-derivable', '--allow-dirty'])
except:
    print("Failed to publish blf_lib-derivable")

print("blf_lib-derive: publishing")
try:
    subprocess.Popen(f'cargo upgrade -p blf_lib-derivable@{new_version} --manifest-path blf_lib-derive/Cargo.toml --pinned')
    subprocess.call(['cargo', 'publish', '-p', 'blf_lib-derive', '--allow-dirty'])
except:
    print("Failed to publish blf_lib-derive")

print("blf_lib: publishing")
try:
    subprocess.Popen(f'cargo upgrade -p blf_lib-derivable@{new_version} -p blf_lib-derive@{new_version} --manifest-path blf_lib/Cargo.toml --pinned')
    subprocess.call(['cargo', 'publish', '-p', 'blf_lib', '--allow-dirty'])
except:
    print("Failed to publish blf_lib")

print("blf_cli: publishing")
try:
    subprocess.Popen(f'cargo upgrade -p blf_lib@{new_version} --manifest-path blf_cli/Cargo.toml --pinned')
    subprocess.call(['cargo', 'publish', '-p', 'blf_lib', '--allow-dirty'])
except:
    print("Failed to publish blf_cli")

subprocess.call("git add Cargo.toml")
subprocess.call(f"git commit -m \"Publish {new_version}\"")
subprocess.call(f"git tag v{new_version}")
subprocess.call("git push")
subprocess.call(f"git push origin v{new_version}")
