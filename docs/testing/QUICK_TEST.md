# Browser Control - Quick Test (30 seconds)

## Step 1: Open Frontend
```
http://localhost:3000
```

## Step 2: Test These 5 Commands (copy-paste into chat)

```
system browser help
```
✅ Expected: List of browser commands

```
system browser sessions
```
✅ Expected: "Found X browser sessions" or "No sessions found"

```
system browser launch chrome
```
✅ Expected: Chrome window opens, chat shows "Launched chrome on port 9222"

```
system browser navigate | url=https://news.ycombinator.com
```
✅ Expected: Browser goes to HN, chat shows "Navigated to..."

```
system browser scrape | selector=.titleline
```
✅ Expected: Article titles extracted and displayed in chat

## Done!

If all 5 work → ✅ **Browser control is fully operational**

Issues? Check: `BROWSER_TEST_RESULTS.md` or `docs/BROWSER_CONTROL_TESTING.md`
