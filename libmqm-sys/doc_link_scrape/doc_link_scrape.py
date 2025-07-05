import requests
from bs4 import BeautifulSoup
import csv
import re

def scrape_mq_links_multiple_pages(urls, output_filename="mq_links_combined.csv"):
    """
    Scrapes links from multiple webpages, filtering for links where the text starts with "MQ",
    and saves them to a single CSV file.

    Args:
        urls (list): A list of URLs of the webpages to scrape.
        output_filename (str): The name of the CSV file to save the links to.
    """
    all_mq_links = []
    
    for url in urls:
        print(f"\nAttempting to scrape links from: {url}")
        try:
            headers = {"User-Agent": "curl/8.5.0"}
            # Fetch the webpage content, allowing redirects
            response = requests.get(url, allow_redirects=True,headers=headers)
            response.raise_for_status()  # Raise an exception for HTTP errors (4xx or 5xx)

            # Parse the HTML content
            soup = BeautifulSoup(response.text, 'html.parser')

            # Find all anchor tags (links)
            links = soup.find_all('a', string=re.compile(".*\\bmq\\w+\\b.*", flags=re.IGNORECASE))

            # Filter links by text content starting with "MQ"
            found_on_page = 0
            for link in links:
                link_text = link.get_text(strip=True)
                link_href = link.get('href')

                # Construct absolute URL if it's a relative one
                if not re.match(r'^[a-z]+://', link_href):
                    link_href = requests.compat.urljoin(url, link_href)
                all_mq_links.append({"name": link_text, "url": link_href})
                found_on_page += 1
            
            print(f"  Found {found_on_page} links starting with 'MQ' on this page.")

        except requests.exceptions.RequestException as e:
            print(f"  Error fetching the URL {url}: {e}")
        except Exception as e:
            print(f"  An unexpected error occurred while processing {url}: {e}")

    # Write all the filtered links to a single CSV file
    if all_mq_links:
        with open(output_filename, 'w', newline='', encoding='utf-8') as csvfile:
            fieldnames = ['name', 'url']
            writer = csv.DictWriter(csvfile, fieldnames=fieldnames)

            writer.writeheader()
            for link_data in all_mq_links:
                writer.writerow(link_data)
        print(f"\nSuccessfully scraped a total of {len(all_mq_links)} links from all pages and saved to {output_filename}")
    else:
        print(f"\nNo links found starting with 'MQ' across any of the provided URLs. No CSV file created.")

if __name__ == "__main__":
    # --- Configuration ---
    # **IMPORTANT: Replace with the actual URLs you want to scrape**
    target_urls = [
        "https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101650_.html",
        "https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q093560_.html",
        "https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109600_.html",
        "https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q104060_.html",
        "https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q108060_.html",
        "https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q108170_.html",
        "https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089140_.html",
        "https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q088570_.html",
        "https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q049370_.html",
        "https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110070_.html",
        "https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q108130_.html",
        "https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q082490_.html",
        "https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109670_.html",
        "https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q040700_.html",
        "https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046040_.html",
    ]
    output_csv_file = "mq_filtered_links_combined.csv"

    print("Starting multi-page link scraping...")
    scrape_mq_links_multiple_pages(target_urls, output_csv_file)
    print("Scraping process finished.")