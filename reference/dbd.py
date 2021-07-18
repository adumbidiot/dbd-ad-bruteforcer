import subprocess

base_key = 'ytsmtqofpvu'

def doit(key, index):
    if len(base_key) <= index:
        return
    
    try:
        subprocess.run(f'youtube-dl https://www.youtube.com/watch?v={key}', shell=True, check=True)
    except subprocess.CalledProcessError:
        pass
    
    key1 = ''.join((key[:index], key[index].upper(), key[index+1:]))
    try:
        subprocess.run(f'youtube-dl https://www.youtube.com/watch?v=' + key1, shell=True, check=True)
    except subprocess.CalledProcessError:
        pass
        
        
    doit(key, index + 1)
    doit(key1, index + 1)
    
doit(base_key, 0)

print('DONE')