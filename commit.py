import subprocess

print('Enter a commit message.')
commit_message = input()
subprocess.run('cargo fmt')
subprocess.run('git add *')
subprocess.run(f'git commit -m \"{commit_message}\"')
subprocess.run('git push origin main')
