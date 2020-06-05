# assg4

Basics of machine learning based on text classification using the Scikit-learn library.

## Overview

This script is a university assignment for the Artificial intelligence and knowledge engineering course. It is a system for training and testing various machine learning models on Wikipedia articles. The dataset consists of about 10000 articles in Polish, which come from 34 different categories.

For the purposes of the assignment, two main tested classification models were the [naive Bayes classifier](https://en.wikipedia.org/wiki/Naive_Bayes_classifier) and the [decision tree classifier](https://en.wikipedia.org/wiki/Decision_tree_learning), but the script could be easily extended to include other classifiers.

The system provides basic feature extraction based on word stemming, as well as feature selection with the help of chi2 and the TF-IDF statistic. The system uses k-fold cross-validation to determine the best model, which is in turn being evaluated.

## Installation

- To fetch the necessary data run
  ```bash
  ./scripts/fetch.sh
  ```
- Run the main script with
  ```bash
  python3 src/main.py data/
  ```
  where `data` is the directory containing the fetched articles.
