export default function ({ $auth, redirect }: any) {
    if (!$auth.user.value) {
      return redirect('/login')
    }
  }